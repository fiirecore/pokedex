use crate::MoveRef;

use super::PokemonMove;

pub type MoveInstances = Vec<MoveInstance>;

pub struct MoveInstance {
	
	pub pokemon_move: MoveRef,
	pub remaining_pp: u8,
	
}

impl MoveInstance {

	pub fn use_move(&mut self) -> &PokemonMove {
		self.remaining_pp -= 1;
		&self.pokemon_move
	}

}