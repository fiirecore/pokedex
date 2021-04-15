use serde::{Deserialize, Serialize};

use super::pokemon::types::PokemonType;

pub mod saved;

pub type MoveId = u16;
pub type PP = u8;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PokemonMove {

	pub id: MoveId,

	pub name: String,
	pub category: MoveCategory,
	#[serde(rename = "type")]
	pub pokemon_type: PokemonType,

	pub power: Option<u8>,
	pub accuracy: Option<u8>,
	pub pp: PP,
	
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum MoveCategory {
	Physical,
	Special,
	Status,	
}

impl std::fmt::Display for PokemonMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}