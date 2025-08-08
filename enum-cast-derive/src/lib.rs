use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Ident, parse_macro_input};

#[proc_macro_derive(EnumCast)]
pub fn enum_cast_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let enum_name = &input.ident;

    let variants = match &input.data {
        Data::Enum(data_enum) => &data_enum.variants,
        _ => panic!("EnumCast can only be derived for enums"),
    };

    let mut variant_info = Vec::new();

    // Extract variant information
    for variant in variants {
        let variant_name = &variant.ident;

        let field = if let Fields::Unnamed(fields) = &variant.fields {
            if fields.unnamed.len() != 1 {
                panic!(
                    "EnumCast can only be derived for enums with single unnamed fields in variants"
                );
            }
            &fields.unnamed[0]
        } else {
            panic!("EnumCast can only be derived for enums with unnamed fields in variants");
        };

        let field_type = &field.ty;
        variant_info.push((variant_name, field_type));
    }

    let contains_impls = derive_contains_impls(enum_name, &variant_info);
    let is_subset_impl = derive_is_subset_of_impl(enum_name, &variant_info);
    let extension_impl = derive_extension(enum_name);

    let expanded = quote! {
        #(#contains_impls)*
        #is_subset_impl
        #extension_impl
    };

    TokenStream::from(expanded)
}

fn derive_contains_impls(
    enum_name: &Ident,
    variant_info: &[(&Ident, &syn::Type)],
) -> Vec<proc_macro2::TokenStream> {
    let mut contains_impls = Vec::new();

    for &(variant_name, field_type) in variant_info {
        let contains_impl = quote! {
            impl ::enum_cast::Contains<#field_type> for #enum_name {
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
        contains_impls.push(contains_impl);
    }

    contains_impls
}

fn derive_is_subset_of_impl(
    enum_name: &Ident,
    variant_info: &[(&Ident, &syn::Type)],
) -> proc_macro2::TokenStream {
    let other_bounds = variant_info.iter().map(|&(_variant_name, field_type)| {
        quote! { ::enum_cast::Contains<#field_type> }
    });

    let widen_arms = variant_info.iter().map(|&(variant_name, _field_type)| {
        quote! {
            #enum_name::#variant_name(v) => Other::make(v)
        }
    });

    let narrow_chain = generate_narrow_chain(variant_info, enum_name);

    quote! {
        impl<Other: #(#other_bounds)+*> ::enum_cast::IsSubsetOf<Other> for #enum_name {
            fn widen(self) -> Other {
                match self {
                    #(#widen_arms,)*
                }
            }

            fn narrow_from(other: Other) -> Result<Self, Other> {
                #narrow_chain
            }
        }
    }
}

fn generate_narrow_chain(
    variant_info: &[(&Ident, &syn::Type)],
    enum_name: &Ident,
) -> proc_macro2::TokenStream {
    if variant_info.is_empty() {
        return quote! { Err(other) };
    }

    let mut result = quote! { Err(other) };

    // Build the chain from right to left (last to first)
    for &(variant_name, _field_type) in variant_info.iter().rev() {
        result = quote! {
            match other.take() {
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
            fn widen<Other>(self) -> Other
            where
                Self: ::enum_cast::IsSubsetOf<Other>,
            {
                <Self as ::enum_cast::IsSubsetOf<Other>>::widen(self)
            }

            fn narrow<Other>(self) -> Result<Other, Self>
            where
                Other: ::enum_cast::IsSubsetOf<Self>,
            {
                <Other as ::enum_cast::IsSubsetOf<Self>>::narrow_from(self)
            }
        }
    }
}
