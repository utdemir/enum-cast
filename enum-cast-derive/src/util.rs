use syn::{Data, DeriveInput, Fields, Ident, spanned::Spanned as _};

pub fn get_enum_variant_infos(
    input: &DeriveInput,
) -> Result<Vec<(&Ident, &syn::Type)>, syn::Error> {
    let variants = match &input.data {
        Data::Enum(data_enum) => &data_enum.variants,
        _ => {
            return Err(syn::Error::new_spanned(
                input,
                "EnumCast can only be derived for enums",
            ));
        }
    };

    let mut variant_info = Vec::new();

    // Extract variant information
    for variant in variants {
        let variant_name = &variant.ident;

        let field = match &variant.fields {
            Fields::Unnamed(fields) => {
                if fields.unnamed.len() != 1 {
                    return Err(syn::Error::new(
                        fields.span(),
                        "EnumCast can only be derived for enums with a single unnamed field in variants",
                    ));
                }
                &fields.unnamed[0]
            }
            Fields::Unit | Fields::Named(..) => {
                return Err(syn::Error::new(
                    variant.fields.span(),
                    "EnumCast can only be derived for enums with a single unnamed field in variants",
                ));
            }
        };

        let field_type = &field.ty;
        variant_info.push((variant_name, field_type));
    }

    Ok(variant_info)
}
