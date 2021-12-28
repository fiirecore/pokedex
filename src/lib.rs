//! Basic pokedex library written in Rust.
//!

#![deny(unsafe_code)]
// #![deny(missing_docs)]

pub mod ailment;
pub mod item;
pub mod moves;
pub mod pokemon;
pub mod types;

mod dex;
pub use dex::*;

mod id;
pub use id::*;

/// A trait that helps initialize values with a Dex.
pub trait Initializable<'d, I: Identifiable, O: core::ops::Deref<Target = I>> {
    /// The output of initialization.
    type Output;

    /// The function to initialize this value.
    fn init(self, initializer: &'d dyn Dex<'d, I, O>) -> Option<Self::Output>;
}

/// A trait that helps uninitialize values (mostly into a non-borrowing form).
pub trait Uninitializable {
    /// The uninitialized value.
    type Output;

    /// The function to uninitialize this value.
    fn uninit(self) -> Self::Output;
}
