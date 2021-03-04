use serde::{Deserialize, Serialize};

use crate::Level;
use crate::pokemon::data::StatSet;
use crate::pokemon::Gender;

use crate::PokemonId;
use crate::moves::serializable::SerializableMoveSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonInstance {

	pub id: PokemonId,
    pub nickname: Option<String>,
    pub level: Level,
    pub gender: Option<Gender>, // optional because it hasn't been calculated yet (some pokemon arent 50/50 gender)
    
    #[serde(default = "iv_default")]
	pub ivs: StatSet,
    #[serde(default)]
    pub evs: StatSet,
    
    pub moves: Option<SerializableMoveSet>,
    
    #[serde(default)]
	pub exp: usize,
    #[serde(default)]
    pub friendship: u8,
    
    pub current_hp: Option<u16>,

}

impl PokemonInstance {

    pub fn generate(pokemon_id: PokemonId, min_level: Level, max_level: Level, ivs: Option<StatSet>) -> Self {

        Self {

            id: pokemon_id,
            nickname: None,
            gender: None,
            level: quad_rand::gen_range(min_level, max_level),
            ivs: ivs.unwrap_or_default(),
            evs: StatSet::default(),
            current_hp: None,
            moves: None,
            exp: 0,
            friendship: 70,

        }

    }
    
    pub fn generate_with_level(pokemon_id: PokemonId, level: Level, ivs: Option<StatSet>) -> Self {
        PokemonInstance::generate(pokemon_id, level, level, ivs)
    }

}

const fn iv_default() -> StatSet {
    StatSet {
        hp: 15,
        atk: 15,
        def: 15,
        sp_atk: 15,
        sp_def: 15,
        speed: 15,
    }
}