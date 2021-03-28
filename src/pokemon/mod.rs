use dashmap::mapref::one::Ref;
use firecore_pokedex_lib::pokemon::data::Gender;

use crate::moves::instance::{MoveInstance, MoveInstanceSet};


pub use firecore_pokedex_lib::pokemon::*;


pub mod generate;
pub mod random;

pub mod instance;


pub type PokemonRef = Ref<'static, PokemonId, Pokemon>;



pub trait InPokedex {

    fn moves_from_level(&self, level: Level) -> MoveInstanceSet;

    // #[deprecated(note = "move or rename trait")]
    fn generate_gender(&self) -> Gender;

}

impl InPokedex for Pokemon {

	fn moves_from_level(&self, level: u8) -> MoveInstanceSet {
		let mut moves: Vec<MoveInstance> = Vec::new();
		for learnable_move in &self.moves {
			if learnable_move.level <= level {
				if let Some(pokemon_move) =  crate::movedex().get(&learnable_move.move_id) {
					let mut has = false;
					for pmove in &moves {
						if pmove.pokemon_move.id == pokemon_move.id {
							has = true;
						}
					}
					if !has {
						moves.push(MoveInstance {
							pp: pokemon_move.pp,
							pokemon_move: pokemon_move,
						});
					}
					
				}
			}
		}
		moves.reverse();
		moves.truncate(4);

		return moves.into();		
	}

    fn generate_gender(&self) -> Gender {
        match self.breeding.gender {
            Some(percentage) => if quad_rand::gen_range(0, 8) > percentage {
                Gender::Male
            } else {
                Gender::Female
            }
            None => Gender::None,
        }
    }
	
}