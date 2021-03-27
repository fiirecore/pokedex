use dashmap::mapref::one::Ref;
pub use firecore_pokedex_lib::moves::*;

pub type MoveRef = Ref<'static, MoveId, PokemonMove>;

pub mod instance {

    use firecore_pokedex_lib::moves::{PokemonMove, PP};
    use smallvec::SmallVec;

    use super::MoveRef;


    pub type MoveInstances = SmallVec<[MoveInstance; 4]>;

    pub struct MoveInstance {
        
        pub pokemon_move: MoveRef,
        pub pp: PP,
        
    }
    
    impl MoveInstance {

        pub fn new(pokemon_move: MoveRef) -> Self {
            Self {
                pp: pokemon_move.pp,
                pokemon_move,
            }
        }
    
        pub fn use_move(&mut self) -> Option<&PokemonMove> {
            if self.pp == 0 {
                None
            } else {
                self.pp -= 1;
                Some(&self.pokemon_move)
            }
            
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
            if let Some(pokemon_move) = crate::MOVEDEX.get(&saved_move.id) {
                instances.push(MoveInstance {
                    pokemon_move: pokemon_move,
                    pp: saved_move.pp,
                });
            }
        }
        return instances;
    }
    
    pub fn from_instances(moves: MoveInstances) -> SerializableMoveSet {
        moves.into_iter().map(|instance| SerializableMove {
            id: instance.pokemon_move.id,
            pp: instance.pp,
        }).collect()
    }

}