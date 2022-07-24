//! Basic pokedex library written in Rust.
//!

#![cfg_attr(not(test), no_std)]
#![deny(unsafe_code)]

extern crate alloc;
// #![deny(missing_docs)]


mod dex;
pub use dex::*;

mod id;
pub use id::*;

pub mod ailment;
pub mod item;
pub mod moves;
pub mod pokemon;
pub mod types;
pub mod trainer;

pub type Money = u32;
