pub use firecore_pokedex_lib::moves::*;

pub mod instance {

    use firecore_pokedex_lib::moves::PokemonMove;
    use smallvec::SmallVec;

    use crate::MoveRef;


    pub type MoveInstances = SmallVec<[MoveInstance; 4]>;

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

}



pub mod serializable {
    
    use firecore_pokedex_lib::moves::{SerializableMove, SerializableMoveSet};
    use smallvec::SmallVec;

    use super::instance::{MoveInstance, MoveInstances};

    pub fn to_instances(moves: &SerializableMoveSet) -> MoveInstances {
        let mut instances = SmallVec::new();
        for saved_move in moves {
            if let Some(pokemon_move) = crate::MOVEDEX.get(&saved_move.move_id) {
                instances.push(MoveInstance {
                    pokemon_move: pokemon_move,
                    remaining_pp: saved_move.remaining_pp,
                });
            } else {
                // macroquad::prelude::warn!("Could not get pokemon move from id {}!", saved_move.move_id);
            }
        }
        return instances;
    }
    
    pub fn from_instances(moves: &MoveInstances) -> SerializableMoveSet {
        moves.iter().map(|instance| SerializableMove {
            move_id: instance.pokemon_move.number,
            remaining_pp: instance.remaining_pp,
        }).collect()
    }

}