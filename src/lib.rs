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
    type Id: PartialEq + Clone;

    /// The identifier to fallback to when an unknown value is needed.
    const UNKNOWN: Self::Id;

    /// Get the identifier of this value.
    fn id(&self) -> &Self::Id;

    /// The name of this value.
    fn name(&self) -> &str;
}

/// A trait that helps initialize values with a Dex.
pub trait Initializable<'d, I: Identifiable> {
    /// The output of initialization.
    type Output;

    /// The function to initialize this value.
    fn init(self, initializer: &'d dyn Dex<I>) -> Option<Self::Output>;
}

/// A trait that helps uninitialize values (mostly into a non-borrowing form).
pub trait Uninitializable {
    /// The uninitialized value.
    type Output;

    /// The function to uninitialize this value.
    fn uninit(self) -> Self::Output;
}

// #[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
// pub struct MaximumNumber<N: Restorable, D, I: Identifiable>(pub N, #[serde(skip)] Option<N>);

// impl<N: Restorable + serde::Serialize + serde::de::DeserializeOwned, D, I: Identifiable> MaximumNumber<N, D, I> {

//     pub fn of(n: N) -> Self {
//         Self(n, None)
//     }

//     pub fn restore(&mut self) {
//         self.0 = self.1.unwrap_or_else(N::max)
//     }

//     pub fn initialize(&mut self, max: N) {
//         self.0 = self.1.unwrap_or(max);
//         self.1 = Some(max);
//     }

//     pub fn add(&mut self, n: N) -> &mut Self {
//         let n = self.0 + n;
//         self.0 = match self.1 {
//             Some(max) => max.min(n),
//             None => n,
//         };
//         self
//     }

//     pub fn sub(&mut self, n: N) -> &mut Self {
//         self.0.saturating_sub(n);
//         self
//     }

// }

// pub trait Restorable: Ord + core::ops::Add<Output = Self> {

//     type Data;
//     type Identifier: Identifiable;

//     fn saturating_sub(&mut self, n: Self);

//     fn get_maximum(data: &Self::Data, value: &Self::Identifier) -> Self;

// }
