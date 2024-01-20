use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{parse_macro_input, parse_quote, FnArg, ImplItem, ItemImpl, Pat, ReturnType, Type, TypePath};
use to_snake_case::ToSnakeCase;
use proc_macro_crate::{crate_name, FoundCrate::*};

#[proc_macro_attribute]
pub fn native(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let crate_name = {
        let found_crate = crate_name("native").expect("No such crate `native`");

        match found_crate {
            Itself => quote!(crate),
            Name(name) => {
                let ident = Ident::new(&name, Span::call_site());
                quote!(#ident)
            }
        }
    };

    let input = parse_macro_input!(item as ItemImpl);

    let Type::Path(struct_ident) = *input.self_ty.clone() else {
        panic!("Native type `{}` is invalid!", input.self_ty.to_token_stream());
    };

    if input.trait_.is_some() {
        panic!("Can't make trait impl to native!");
    }

    let native_functions = input.items.iter().filter_map(|item| {
        if let ImplItem::Fn(func) = item {
            let ident = get_func_ident(&func.sig.ident, &struct_ident);

            let block = func.block.clone();
            let (return_type, func_return_type) = get_return_type(&func.sig.output);
            let return_block = match func_return_type {
                FuncType::StringRef => quote! {
                    let value = #block;

                    CString::new(value).unwrap().into_raw()
                },
                FuncType::ObjectRefMut => quote! {
                    let value = #block;

                    Box::into_raw(Box::new(value))
                },
                FuncType::ObjectRef => quote! {
                    panic!("Cannot return non-mutable object pointer");
                },
                _ => quote! {
                    return #block;
                },
            };

            let mut params = func.sig.inputs.clone();
            let mut ffi_transform = TokenStream2::new();

            for param in &mut params {
                if let FnArg::Typed(ref mut pat_type) = param {
                    if let Pat::Ident(ref mut pat_ident) = *pat_type.pat {
                        let ident = &mut pat_ident.ident;
                        let ty = &mut *pat_type.ty;

                        let func_type = FuncType::try_from(&*ty).unwrap();
                        let new_ident = Ident::new(format!("_{}", ident.clone()).as_str(), Span::call_site());

                        match func_type {
                            FuncType::ObjectRef => {
                                ffi_transform.append_all(quote! {
                                    let #ident = #crate_name::assert_ptr(#new_ident);
                                });

                                *ident = new_ident;
                                *ty = parse_quote!{ *const #ty };
                            },
                            FuncType::ObjectRefMut => {
                                ffi_transform.append_all(quote! {
                                    let #ident = #crate_name::assert_ptr_mut(#new_ident);
                                });

                                *ident = new_ident;
                                *ty = parse_quote!{ *mut #ty };
                            },
                            FuncType::StringRef => {
                                ffi_transform.append_all(quote! {
                                    let #ident = #crate_name::ptr_to_string(#new_ident);
                                });

                                *ident = new_ident;
                                *ty = parse_quote!{ *const c_char };
                            },
                            _ => {
                                ffi_transform.append_all(quote! {
                                    let #ident = #new_ident;
                                });

                                *ident = new_ident;
                            },
                        }
                    } else { 
                        panic!("Invalid function parameter identifier"); 
                    };
                } else {
                    panic!("Self parameter is not supported");
                }
            }

            return Some(quote! {
                #[allow(clippy::missing_safety_doc)]
                #[no_mangle]
                pub unsafe extern "C" fn #ident(#params) #return_type {
                    #ffi_transform
                    #return_block
                }
            });
        }
        
        None
    });

    let free_func_ident = get_func_ident(&Ident::new("free", Span::call_site()), &struct_ident);
    let free_debug_msg = format!("{}::free()", struct_ident.to_token_stream());

    let free_function = quote! {
        #[allow(clippy::missing_safety_doc)]
        #[no_mangle]
        pub unsafe extern "C" fn #free_func_ident(instance: *mut #struct_ident) {
            #crate_name::free_ptr(instance);
            ::flatbox_core::logger::debug!(#free_debug_msg);
        }
    };

    let output = quote! {
        impl #struct_ident {
            #(#native_functions)*
            #free_function
        }
    };

    output.into()
}

#[derive(Debug)]
enum FuncType {
    Unit,
    Primitive,
    StringRef,
    ObjectRef,
    ObjectRefMut,
}

impl TryFrom<&Type> for FuncType {
    type Error = String;

    fn try_from(value: &Type) -> Result<Self, Self::Error> {
        use FuncType::*;

        match value {
            Type::Path(type_path) => {
                if is_primitive(type_path) {
                    return Ok(Primitive);
                }

                Ok(ObjectRefMut)
            },
            Type::Reference(type_ref) => {
                if let Type::Path(ref ref_elem_type) = *type_ref.elem {
                    if ref_elem_type.path.is_ident("str") {
                        return Ok(StringRef)
                    }
                }

                if type_ref.mutability.is_some() {
                    Ok(ObjectRefMut)
                } else {
                    Ok(ObjectRef)
                }
            },
            Type::Tuple(tuple) if tuple.elems.is_empty() => {
                Ok(Unit)
            },
            _ => Err("Non-path and non-reference return types are not supported".to_string()),
        }
    }
}

fn get_func_ident(func_ident: &Ident, struct_ident: &TypePath) -> Ident {
    let struct_ident = struct_ident.to_token_stream().to_string();
    let func_ident = func_ident.to_string();
    if !struct_ident.chars().all(char::is_alphabetic) {
        panic!("Native struct name `{struct_ident}` is invalid!");
    }

    Ident::new(format!("{}_{}", struct_ident.to_snake_case(), func_ident).as_str(), Span::call_site())
}

fn get_return_type(return_type: &ReturnType) -> (ReturnType, FuncType) {
    use FuncType::*;

    match return_type {
        ReturnType::Type(t, ty) => {
            match *ty.clone() {
                Type::Path(type_path) => {
                    if is_primitive(&type_path) {
                        return (return_type.clone(), Primitive);
                    }

                    let ty = Box::new(parse_quote! { *mut #type_path });

                    (ReturnType::Type(*t, ty), ObjectRefMut)
                },
                Type::Reference(type_ref) => {
                    if let Type::Path(ref_elem_type) = *type_ref.elem {
                        if ref_elem_type.path.is_ident("str") {
                            let ty = Box::new(parse_quote! { *const c_char });

                            return (ReturnType::Type(*t, ty), StringRef);
                        }
                    }

                    panic!("Only `&str` reference types are supported");
                },
                Type::Tuple(tuple) if tuple.elems.is_empty() => {
                    (return_type.clone(), Unit)
                },
                _ => panic!("Non-path and non-reference return types are not supported"),
            }
        },
        _ => (ReturnType::Default, Unit),
    }
}

fn is_primitive(type_path: &TypePath) -> bool {
    type_path.path.is_ident("i8")
        || type_path.path.is_ident("i16")
        || type_path.path.is_ident("i32")
        || type_path.path.is_ident("i64")
        || type_path.path.is_ident("i128")
        || type_path.path.is_ident("u8")
        || type_path.path.is_ident("u16")
        || type_path.path.is_ident("u32")
        || type_path.path.is_ident("u64")
        || type_path.path.is_ident("u128")
        || type_path.path.is_ident("f32")
        || type_path.path.is_ident("f64")
        || type_path.path.is_ident("char")
        || type_path.path.is_ident("bool")
}