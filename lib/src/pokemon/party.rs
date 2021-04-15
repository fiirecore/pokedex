// use serde::{Deserialize, Serialize};
use util::smallvec::SmallVec;

use super::saved::SavedPokemon;

pub type PokemonParty = SmallVec<[SavedPokemon; 6]>;

// #[derive(Clone, Debug, Default, Serialize, Deserialize)]
// pub struct PokemonParty {

	

// }

// impl PokemonParty {

// 	pub fn to_instance(&self, pokedex: &super::pokedex::Pokedex) -> Vec<PokemonInstance> {
// 		self.pokemon.iter().map(|pkmn| pkmn.to_pokemon(pokedex)).collect()
// 	}

// }