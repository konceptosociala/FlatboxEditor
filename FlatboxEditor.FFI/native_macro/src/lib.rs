mod func_type;
mod native_funcs;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ImplItem, ItemImpl, Type};

use crate::func_type::FuncType;
use crate::native_funcs::*;

#[proc_macro_attribute]
pub fn native(_: TokenStream, item: TokenStream) -> TokenStream {
    let crate_name = get_crate_name();

    let input = parse_macro_input!(item as ItemImpl);

    let Type::Path(struct_ident) = *input.self_ty.clone() else {
        panic!("Native type `{}` is invalid!", input.self_ty.to_token_stream());
    };

    if input.trait_.is_some() {
        panic!("Can't make trait impl to native!");
    }

    let functions = input.items.iter().filter_map(|item| { 
        if let ImplItem::Fn(func) = item { 
            Some(func.clone()) 
        } else { 
            None 
        } 
    }).collect::<Vec<_>>();
    
    let native_functions = functions.iter().map(|inner_func| {
        let ident = get_func_ident(&inner_func.sig.ident, &struct_ident);

        let inner_func_ident = inner_func.sig.ident.clone();
        let inner_func_params = get_inner_func_params(&inner_func.sig.inputs);
        let (return_type, func_return_type) = get_return_type(&inner_func.sig.output);
        let return_block = match func_return_type {
            FuncType::StringRef => quote! {
                let value = #struct_ident::#inner_func_ident(#inner_func_params);

                CString::new(value).unwrap().into_raw()
            },
            FuncType::Object => quote! {
                let value = #struct_ident::#inner_func_ident(#inner_func_params);

                Box::into_raw(Box::new(value))
            },
            _ => quote! {
                return #struct_ident::#inner_func_ident(#inner_func_params);
            },
        };

        let (params, ffi_transform) = get_ffi_transform(&inner_func.sig.inputs, &crate_name);

        Some(quote! {
            /// Sas
            #[allow(clippy::missing_safety_doc)]
            #[no_mangle]
            pub unsafe extern "C" fn #ident(#params) #return_type {
                #ffi_transform
                #return_block
            }
        })
    });

    let free_function = get_free_function(&struct_ident, &crate_name);

    let output = quote! {
        impl #struct_ident {
            #(#functions)*
            #(#native_functions)*
            #free_function
        }
    };

    output.into()
}