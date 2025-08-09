pub use enum_cast_derive::{EnumCast, EnumVariantIds};
pub use typeid::ConstTypeId;

pub trait Contains<T>
where
    Self: Sized,
{
    fn make(t: T) -> Self;
    fn take(self) -> Result<T, Self>;
}

pub trait IsSubsetOf<Other>
where
    Self: Sized,
{
    fn upcast(self) -> Other;
    fn downcast_from(other: Other) -> Result<Self, Other>;
}

pub trait EnumVariantIds {
    const VARIANT_TYPE_IDS: &'static [ConstTypeId];
    fn current_variant_id(&self) -> ConstTypeId;
}