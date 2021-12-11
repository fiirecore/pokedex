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

/// An ascii string that holds the value "unknown"
#[allow(unsafe_code)]
pub const UNKNOWN_ID: tinystr::TinyStr16 =
    unsafe { tinystr::TinyStr16::new_unchecked(31093567915781749) };

/// A trait that helps identify which value of a type is which.
pub trait Identifiable {
    /// The type that identifies this type.
    type Id;

    /// The identifier to fallback to when an unknown value is needed.
    const UNKNOWN: Self::Id;

    /// Get the identifier of this value.
    fn id(&self) -> &Self::Id;

    /// The name of this value.
    fn name(&self) -> &str;
}

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

// mod identifiable {
//     use std::iter::FromIterator;

//     use crate::Identifiable;

//     impl<I: Identifiable, C: FromIterator<(I::Id, I)>> FromIterator<I> for C where I::Id: Clone {
//         fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
//             iter.into_iter().map(|i| (i.id().clone(), i)).collect()
//         }
//     }
// }
