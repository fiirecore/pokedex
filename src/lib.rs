//! Basic pokedex library written in Rust.
//! 


pub mod ailment;
pub mod item;
pub mod moves;
pub mod pokemon;
pub mod types;

mod dex;
pub use dex::*;

pub mod id;
pub use id::IdRef;

fn name<T: ?Sized>() -> &'static str {
    let name = core::any::type_name::<T>();
    name.split("::").last().unwrap_or(name)
}

/// A trait that helps identify which value of a type is which.
pub trait Identifiable {
    /// The type that identifies this type.
    type Id: serde::de::DeserializeOwned + serde::Serialize + core::fmt::Display + Copy + Eq + core::hash::Hash;

    /// The identifier to fallback to when an unknown value is needed.
    const UNKNOWN: Self::Id;

    /// Get the identifier of this type.
    fn id(&self) -> &Self::Id;
}
