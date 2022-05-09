//! Pokemon party structs/traits/constants.
//!
//! This module is incomplete and due to change.

/// Common maximum size of a Pokemon party.
pub const PARTY_SIZE: usize = 6;

// /// Unimplemented.
// pub trait PartyTrait {
//     const SIZE: usize = PARTY_SIZE;
// }

pub use defaults::*;

mod defaults {
    use super::PARTY_SIZE;

    /// A type that represents a Pokemon party.
    /// A Party is a collection of owned pokemon a trainer can use.
    pub type Party<P> = alloc::vec::Vec<P>;
}
