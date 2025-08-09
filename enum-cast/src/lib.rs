pub use enum_cast_derive::{EnumCast, EnumVariantIds};
pub use typeid::ConstTypeId;

/// A trait for enums that contain a variant holding a value of type `T`.
pub trait HasVariant<T>
where
    Self: Sized,
{
    fn make(t: T) -> Self;
    fn take(self) -> Result<T, Self>;
}

/// A trait indicating that `Self` is a subset of `Other`, meaning all variants of `Self` exist in `Other`.
pub trait IsSubsetOf<Other>
where
    Self: Sized,
{
    /// Converts `Self` to `Other` by casting to the equivalent variant.
    fn upcast(self) -> Other;

    /// Attempts to convert from `Other` to `Self` if the variant matches.
    /// Returns the original `Other` if no matching variant is found.
    fn downcast_from(other: Other) -> Result<Self, Other>;
}

pub trait EnumVariantIds {
    const VARIANT_TYPE_IDS: &'static [ConstTypeId];
    fn current_variant_id(&self) -> ConstTypeId;
}
