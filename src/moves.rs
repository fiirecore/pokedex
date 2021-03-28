use dashmap::mapref::one::Ref;
pub use firecore_pokedex_lib::moves::*;

pub type MoveRef = Ref<'static, MoveId, PokemonMove>;

pub mod instance {

    use firecore_pokedex_lib::moves::{PokemonMove, PP};
    use smallvec::SmallVec;

    use super::MoveRef;


    pub type MoveInstanceSet = SmallVec<[MoveInstance; 4]>;

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
    
    use firecore_pokedex_lib::moves::saved::{SavedMove, SavedMoveSet};

    use super::instance::{MoveInstance, MoveInstanceSet};

    pub fn to_instances(moves: &SavedMoveSet) -> MoveInstanceSet {
        let mut instances = MoveInstanceSet::new();
        for saved_move in moves {
            if let Some(pokemon_move) = crate::movedex().get(&saved_move.id) {
                instances.push(MoveInstance {
                    pp: saved_move.pp.unwrap_or(pokemon_move.pp),
                    pokemon_move: pokemon_move,
                });
            }
        }
        return instances;
    }
    
    pub fn from_instances(moves: MoveInstanceSet) -> SavedMoveSet {
        moves.into_iter().map(|instance| SavedMove {
            id: instance.pokemon_move.id,
            pp: Some(instance.pp),
        }).collect()
    }

}