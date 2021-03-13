use serde::{Deserialize, Serialize};

use crate::moves::PokemonMove;
use crate::pokemon::Pokemon;

#[derive(Deserialize, Serialize)]
pub struct SerializedDex {
	pub pokemon: Vec<SerializedPokemon>,
	pub moves: Vec<PokemonMove>,
}

#[derive(Deserialize, Serialize)]
pub struct SerializedPokemon {

    pub pokemon: Pokemon,
    pub cry_ogg: Vec<u8>,
    pub front_png: Vec<u8>,
    pub back_png: Vec<u8>,

}

// #[derive(Deserialize, Serialize)]
// pub struct SerializedPokemonMove {

//     pub pokemon_move: PokemonMove,

// }