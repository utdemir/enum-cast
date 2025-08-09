//! `enum-cast` is a library to create ergonomic and safe casting between
//! enums with common variants.
//!
//! Example:
//!
//! ```rust,ignore
//! use enum_cast::EnumCast;
//!
//! #[derive(EnumCast)]
//! enum Foo {
//!     FooA(SomeA),
//!     FooB(SomeB),
//! }
//!
//! #[derive(EnumCast)]
//! enum Bar {
//!     BarA(SomeA),
//!     BarB(SomeB),
//!     BarC(SomeC),
//! }
//!
//! let foo = Foo::FooA(SomeA);
//! assert_eq!(
//!   foo.upcast::<Bar>(),
//!   Bar::BarA(SomeA)
//! );
//!
//! let bar = Bar::BarA(SomeA);
//! assert_eq!(
//!   bar.downcast::<Foo>(),
//!   Ok(Foo::FooA(SomeA))
//! );
//! ```

#[doc(inline)]
/// Derives traits for casting across enums, and implements `downcast` and `upcast` helper methods.
pub use enum_cast_derive::EnumCast;

#[doc(inline)]
/// Derives [EnumVariantIds] trait.
pub use enum_cast_derive::EnumVariantIds;

/// A trait for enums that contain a variant holding a value of type `T`.
///
/// Derived by `EnumCast`
pub trait HasVariant<T>
where
    Self: Sized,
{
    fn make(t: T) -> Self;
    fn take(self) -> Result<T, Self>;
}

/// A trait indicating that `Self` is a subset of `Other`, meaning all variants of `Self` exist in `Other`.
///
/// Derived by `EnumCast`
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

/// Provides runtime access to the type ids of enum variants.
///
/// Derived by `EnumVariantIds`
pub trait EnumVariantIds {
    /// Returns a vector of the type ids of all variants.
    fn variant_type_ids() -> Vec<std::any::TypeId>;
    // TODO: When TypeId::of starts working in const context[1],
    // we should make this an associated constant instead.
    // [1]: https://github.com/rust-lang/rust/issues/77125

    /// Returns the type id of the current variant.
    fn current_variant_id(&self) -> std::any::TypeId;
}
