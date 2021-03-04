use serde::{Deserialize, Serialize};
use data::breeding::Breeding;
use data::LearnableMove;
use data::PokedexData;
use data::StatSet;
use data::training::Training;
use crate::moves::instance::{MoveInstance, MoveInstances};

use self::cry::PokemonCry;

pub mod data;
pub mod types;
pub mod cry;

pub mod instance;
pub mod battle;

pub mod party;
pub mod texture;

#[derive(Serialize, Deserialize)]
pub struct Pokemon {
	
	pub data: PokedexData,
	pub base: StatSet,
	pub moves: Vec<LearnableMove>,
	pub training: Training,
	pub breeding: Breeding,
	#[serde(default)]
	pub cry: PokemonCry,
	
}

impl Pokemon {

	pub fn moves_from_level(&self, level: u8) -> MoveInstances {
		let mut moves: Vec<MoveInstance> = Vec::new();
		for learnable_move in &self.moves {
			if learnable_move.level <= level {
				if let Some(pokemon_move) =  crate::MOVEDEX.get(&learnable_move.move_id) {
					let mut has = false;
					for pmove in &moves {
						if pmove.pokemon_move.number == pokemon_move.number {
							has = true;
						}
					}
					if !has {
						moves.push(MoveInstance {
							remaining_pp: pokemon_move.pp,
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

    pub fn generate_gender(&self) -> Gender {
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

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum Gender {
	
	None,
	Male,
	Female,
	
}