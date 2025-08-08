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
    fn upcast(self) -> Other;
    fn downcast_from(other: Other) -> Result<Self, Other>;
}
