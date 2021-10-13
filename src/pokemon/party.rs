
/// Common maximum size of a Pokemon party.
pub const PARTY_SIZE: usize = 6;

pub trait PartyTrait {
    const SIZE: usize = PARTY_SIZE;
}

pub use defaults::*;


mod defaults {
    use super::PARTY_SIZE;


    /// A type that represents a Pokemon party.
    /// A Party is a collection of pokemon a trainer can use.
    pub type Party<P> = arrayvec::ArrayVec<[P; PARTY_SIZE]>;
}
