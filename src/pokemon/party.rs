use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

use crate::pokemon::instance::PokemonInstance;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PokemonParty {

	pub pokemon: SmallVec<[PokemonInstance; 6]>,

}

// impl PokemonParty {

// 	pub fn to_instance(&self, pokedex: &super::pokedex::Pokedex) -> Vec<PokemonInstance> {
// 		self.pokemon.iter().map(|pkmn| pkmn.to_pokemon(pokedex)).collect()
// 	}

// }