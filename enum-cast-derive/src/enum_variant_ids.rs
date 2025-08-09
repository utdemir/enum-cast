use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

use crate::util::get_enum_variant_infos;

pub fn enum_variant_ids_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    let enum_name = &input.ident;
    
    let variant_info = match get_enum_variant_infos(&input) {
        Ok(info) => info,
        Err(err) => return err.to_compile_error().into(),
    };

    let enum_variant_ids_impl = derive_enum_variant_ids_impl(enum_name, &variant_info);

    let expanded = quote! {
        #enum_variant_ids_impl
    };

    TokenStream::from(expanded)
}

fn derive_enum_variant_ids_impl(
    enum_name: &Ident,
    variant_info: &[(&Ident, &syn::Type)],
) -> proc_macro2::TokenStream {
    let variant_type_ids = variant_info.iter().map(|&(_, ty)| {
        quote! { ::enum_cast::ConstTypeId::of::<#ty>() }
    });

    let current_variant_id_arms = variant_info.iter().map(|&(variant_name, field_type)| {
        quote! {
            #enum_name::#variant_name(_) => ::enum_cast::ConstTypeId::of::<#field_type>()
        }
    });

    quote! {
        impl ::enum_cast::EnumVariantIds for #enum_name {
            const VARIANT_TYPE_IDS: &'static [::enum_cast::ConstTypeId] = &[
                #(#variant_type_ids,)*
            ];

            fn current_variant_id(&self) -> ::enum_cast::ConstTypeId {
                match self {
                    #(#current_variant_id_arms,)*
                }
            }
        }
    }
}