use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident, parse_macro_input};

use crate::util::get_enum_variant_infos;

pub fn enum_cast_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let enum_name = &input.ident;

    let variant_info = match get_enum_variant_infos(&input) {
        Ok(info) => info,
        Err(err) => return err.to_compile_error().into(),
    };

    let has_variant_impls = derive_has_variant_impls(enum_name, &variant_info);
    let is_subset_impl = derive_is_subset_of_impl(enum_name, &variant_info);
    let extension_impl = derive_extension(enum_name);

    let expanded = quote! {
        #(#has_variant_impls)*
        #is_subset_impl
        #extension_impl
    };

    TokenStream::from(expanded)
}

fn derive_has_variant_impls(
    enum_name: &Ident,
    variant_info: &[(&Ident, &syn::Type)],
) -> Vec<proc_macro2::TokenStream> {
    let mut has_variant_impls = Vec::new();

    for &(variant_name, field_type) in variant_info {
        let has_variant_impl = quote! {
            impl ::enum_cast::HasVariant<#field_type> for #enum_name {
                fn make(t: #field_type) -> Self {
                    #enum_name::#variant_name(t)
                }

                fn take(self) -> Result<#field_type, Self> {
                    if let #enum_name::#variant_name(v) = self {
                        Ok(v)
                    } else {
                        Err(self)
                    }
                }
            }
        };
        has_variant_impls.push(has_variant_impl);
    }

    has_variant_impls
}

fn derive_is_subset_of_impl(
    enum_name: &Ident,
    variant_info: &[(&Ident, &syn::Type)],
) -> proc_macro2::TokenStream {
    let other_bounds = variant_info.iter().map(|&(_variant_name, field_type)| {
        quote! { ::enum_cast::HasVariant<#field_type> }
    });

    let upcast_arms = variant_info.iter().map(|&(variant_name, _field_type)| {
        quote! {
            #enum_name::#variant_name(v) => Other::make(v)
        }
    });

    let downcast_chain = generate_downcast_chain(variant_info, enum_name);

    quote! {
        impl<Other: #(#other_bounds)+*> ::enum_cast::IsSubsetOf<Other> for #enum_name {
            fn upcast(self) -> Other {
                match self {
                    #(#upcast_arms,)*
                }
            }

            fn downcast_from(other: Other) -> Result<Self, Other> {
                #downcast_chain
            }
        }
    }
}

fn generate_downcast_chain(
    variant_info: &[(&Ident, &syn::Type)],
    enum_name: &Ident,
) -> proc_macro2::TokenStream {
    if variant_info.is_empty() {
        return quote! { Err(other) };
    }

    let mut result = quote! { Err(other) };

    // Build the chain from right to left (last to first)
    for &(variant_name, field_type) in variant_info.iter().rev() {
        result = quote! {
            // match other.take() {
            match <Other as ::enum_cast::HasVariant<#field_type>>::take(other) {
                Ok(v) => Ok(#enum_name::#variant_name(v)),
                Err(other) => #result,
            }
        };
    }

    result
}

fn derive_extension(enum_name: &Ident) -> proc_macro2::TokenStream {
    quote! {
        impl #enum_name {
            /// Converts the current enum variant to the equivalent variant in the
            /// target enum. The target enum must be a superset of `Self`.
            fn upcast<Other>(self) -> Other
            where
                Self: ::enum_cast::IsSubsetOf<Other>,
            {
                <Self as ::enum_cast::IsSubsetOf<Other>>::upcast(self)
            }

            /// Attempts to convert the current enum variant to the equivalent variant
            /// in the target enum. The target enum must be a subset of `Self`.
            ///
            /// If the current variant cannot be represented in the target enum,
            /// returns the current variant unchanged as `Err(self)`.
            fn downcast<Other>(self) -> Result<Other, Self>
            where
                Other: ::enum_cast::IsSubsetOf<Self>,
            {
                <Other as ::enum_cast::IsSubsetOf<Self>>::downcast_from(self)
            }
        }
    }
}
