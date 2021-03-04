use serde::Deserialize;
use crate::MoveId;

use super::pokemon::types::PokemonType;

pub mod instance;
pub mod serializable;

#[derive(Default, Debug, Clone, Deserialize)]
pub struct PokemonMove {

	pub number: MoveId,
	pub name: String,
	pub category: MoveCategory,
	pub pokemon_type: Option<PokemonType>,
	pub power: Option<usize>,
	pub accuracy: Option<u8>,
	pub pp: u8,
	
}

impl std::fmt::Display for PokemonMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub enum MoveCategory {
	
	Physical,
	Special,
	Status,	
	
}

impl Default for MoveCategory {
    fn default() -> Self {
        Self::Status
    }
}