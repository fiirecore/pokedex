//! Pokemon party structs/traits/constants.
//!
//! This module is incomplete and due to change.

/// Common maximum size of a Pokemon party.
pub const DEFAULT_PARTY_SIZE: usize = 6;

/// A type that represents a Pokemon party.
/// A Party is a collection of owned pokemon a trainer can use.
pub type Party<P> = alloc::vec::Vec<P>;
