use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    punctuated::Punctuated, 
    token::Comma
};
use syn::{parse_quote, FnArg, Pat, ReturnType, Type, TypePath};
use to_snake_case::ToSnakeCase;
use proc_macro_crate::{crate_name, FoundCrate::*};
use crate::func_type::{is_primitive, FuncType};

pub(crate) fn get_free_function(struct_ident: &TypePath, crate_name: &TokenStream2) -> TokenStream2 {
    let free_func_ident = get_func_ident(&Ident::new("free", Span::call_site()), struct_ident);
    let free_debug_msg = format!("{}::free()", struct_ident.to_token_stream());

    quote! {
        #[allow(clippy::missing_safety_doc)]
        #[no_mangle]
        pub unsafe extern "C" fn #free_func_ident(instance: *mut #struct_ident) {
            #crate_name::free_ptr(instance);
            ::flatbox_core::logger::debug!(#free_debug_msg);
        }
    }
}

pub(crate) fn get_ffi_transform(
    params: &Punctuated<FnArg, Comma>, 
    crate_name: &TokenStream2
) -> (Punctuated<FnArg, Comma>, TokenStream2) {
    let mut params = params.clone();
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
                        let Type::Reference(ref_type) = ty else { panic!("Wrong reference type `{}`", ty.to_token_stream()); };
                        let elem = ref_type.elem.clone();

                        ffi_transform.append_all(quote! {
                            let #ident = #crate_name::assert_ptr(#new_ident);
                        });

                        *ident = new_ident;
                        *ty = parse_quote!{ *const #elem };
                    },
                    FuncType::ObjectRefMut => {
                        let Type::Reference(ref_type) = ty else { panic!("Wrong reference type `{}`", ty.to_token_stream()); };
                        let elem = ref_type.elem.clone();

                        ffi_transform.append_all(quote! {
                            let #ident = #crate_name::assert_ptr_mut(#new_ident);
                        });

                        *ident = new_ident;
                        *ty = parse_quote!{ *mut #elem };
                    },
                    FuncType::Object => {
                        ffi_transform.append_all(quote! {
                            let #ident = #new_ident;
                        });

                        *ident = new_ident;
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

    (params, ffi_transform)
}

pub(crate) fn get_crate_name() -> TokenStream2 {
    let found_crate = crate_name("native").expect("No such crate `native`");

    match found_crate {
        Itself => quote!(crate),
        Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            quote!(#ident)
        }
    }
}

pub(crate) fn get_func_ident(func_ident: &Ident, struct_ident: &TypePath) -> Ident {
    let struct_ident = struct_ident.to_token_stream().to_string();
    let func_ident = func_ident.to_string();
    if !struct_ident.chars().all(char::is_alphabetic) {
        panic!("Native struct name `{struct_ident}` is invalid!");
    }

    Ident::new(format!("{}_{}", struct_ident.to_snake_case(), func_ident).as_str(), Span::call_site())
}

pub(crate) fn get_inner_func_params(fn_args: &Punctuated<FnArg, Comma>) -> Punctuated<Box<Pat>, Comma> {
    let mut punc: Punctuated<Box<Pat>, Comma> = Punctuated::new();
    for fn_arg in fn_args {
        let FnArg::Typed(var) = fn_arg else {
            panic!("Self parameter is not supported");
        };
        punc.push(var.pat.clone());
    }
    punc
}

pub(crate) fn get_return_type(return_type: &ReturnType) -> (ReturnType, FuncType) {
    use FuncType::*;

    match return_type {
        ReturnType::Type(t, ty) => {
            match *ty.clone() {
                Type::Path(type_path) => {
                    if is_primitive(&type_path) {
                        return (return_type.clone(), Primitive);
                    }

                    let ty = Box::new(parse_quote! { *mut #type_path });

                    (ReturnType::Type(*t, ty), Object)
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