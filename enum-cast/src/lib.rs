pub use enum_cast_derive::{EnumCast, EnumVariantIds};
pub use typeid::ConstTypeId;


/// A trait for enumerations (coproducts) that contain a variant of type `T`.
pub trait HasVariant<T>
where
    Self: Sized,
{
    fn make(t: T) -> Self;
    fn take(self) -> Result<T, Self>;
}

/// Whether `Other` contains all variants of `Self`.
pub trait IsSubsetOf<Other>
where
    Self: Sized,
{
    /// Cast `Self` to `Other`.
    fn upcast(self) -> Other;
    
    /// Convert from `Other` to self if the variant is matching,
    /// otherwise return the original `Other`.
    fn downcast_from(other: Other) -> Result<Self, Other>;
}

pub trait EnumVariantIds {
    const VARIANT_TYPE_IDS: &'static [ConstTypeId];
    fn current_variant_id(&self) -> ConstTypeId;
}