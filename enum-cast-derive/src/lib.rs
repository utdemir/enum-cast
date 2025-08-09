mod enum_cast;
mod enum_variant_ids;
mod util;

#[proc_macro_derive(EnumCast)]
pub fn enum_cast_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    enum_cast::enum_cast_derive(input)
}

#[proc_macro_derive(EnumVariantIds)]
pub fn enum_variant_ids_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    enum_variant_ids::enum_variant_ids_derive(input)
}