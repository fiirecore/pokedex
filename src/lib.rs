use dashmap::DashMap as HashMap;

use firecore_pokedex_lib::{
	pokemon::{PokemonId, Pokemon},
	moves::{MoveId, PokemonMove}
};

pub use firecore_pokedex_lib::serialized;

pub mod pokemon;
pub mod moves;

lazy_static::lazy_static! {
	pub static ref POKEDEX: HashMap<PokemonId, Pokemon> = HashMap::new();
	pub static ref MOVEDEX: HashMap<MoveId, PokemonMove> = HashMap::new();
}