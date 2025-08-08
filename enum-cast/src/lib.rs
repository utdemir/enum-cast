pub use enum_cast_derive::EnumCast;

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
    fn widen(self) -> Other;
    fn narrow_from(other: Other) -> Result<Self, Other>;
}
