use crate::Identifiable;

use super::Pokemon;

pub type PokemonSpeciesId = super::PokemonId;

pub struct PokemonSpecies {
    pub id: PokemonSpeciesId,
    pub pokemon: Vec<Pokemon>,
}

impl Identifiable for PokemonSpecies {
    type Id = PokemonSpeciesId;

    const UNKNOWN: Self::Id = 0;

    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn name(&self) -> &str {
        self.pokemon[0].name()
    }
}
