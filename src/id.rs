use core::{fmt::Display, hash::Hash};
use serde::{de::DeserializeOwned, Serialize};
use tinystr::TinyStr16;

mod dex;
pub use dex::*;

mod reference;
pub use reference::*;

pub const UNKNOWN_ID: TinyStr16 = unsafe { TinyStr16::new_unchecked(31093567915781749) };

pub trait Identifiable {
    type Id: DeserializeOwned + Serialize + Display + Copy + Eq + Hash;

    const UNKNOWN: Self::Id;

    fn id(&self) -> &Self::Id;
}
