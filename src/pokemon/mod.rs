use firecore_pokedex_lib::pokemon::data::{Gender, StatSet};
use firecore_rand::Random;
use crate::moves::instance::{MoveInstance, MoveInstanceSet};


pub use firecore_pokedex_lib::pokemon::*;

pub mod instance;


pub type PokemonRef = &'static Pokemon;

pub static POKEMON_RANDOM: Random = Random::new();

pub trait GeneratePokemon {

    fn generate(id: PokemonId, min: Level, max: Level, ivs: Option<StatSet>) -> Self;

    fn generate_with_level(id: PokemonId, level: Level, ivs: Option<StatSet>) -> Self where Self: Sized {
        GeneratePokemon::generate(id, level, level, ivs)
    }

}

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
            Some(percentage) => if POKEMON_RANDOM.gen_range(0..8) as u8 > percentage {
                Gender::Male
            } else {
                Gender::Female
            }
            None => Gender::None,
        }
    }
	
}

pub trait RandomSet {

    fn random() -> Self;

}

impl RandomSet for StatSet {

    fn random() -> Self {
		Self {
			hp: POKEMON_RANDOM.gen_range(0..32) as u8,
			atk: POKEMON_RANDOM.gen_range(0..32) as u8,
			def: POKEMON_RANDOM.gen_range(0..32) as u8,
			sp_atk: POKEMON_RANDOM.gen_range(0..32) as u8,
			sp_def: POKEMON_RANDOM.gen_range(0..32) as u8,
			speed: POKEMON_RANDOM.gen_range(0..32) as u8,
		}
	}

}