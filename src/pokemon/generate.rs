use firecore_pokedex_lib::pokemon::{
    PokemonId,
    Level,
    Gender,
    data::StatSet,
    instance::PokemonInstance,
};

use super::InPokedex;

pub trait Generate {

    fn generate(id: PokemonId, min: Level, max: Level, ivs: Option<StatSet>) -> Self;

    fn generate_with_level(id: PokemonId, level: Level, ivs: Option<StatSet>) -> Self where Self: Sized {
        Generate::generate(id, level, level, ivs)
    }

}

impl Generate for PokemonInstance {

    fn generate(pokemon_id: PokemonId, min_level: Level, max_level: Level, ivs: Option<StatSet>) -> Self {

        let pokemon = crate::POKEDEX.get(&pokemon_id);

        Self {

            id: pokemon_id,
            nickname: None,
            gender: pokemon.map(|pokemon| pokemon.generate_gender()).unwrap_or(Gender::None),
            level: quad_rand::gen_range(min_level, max_level),
            ivs: ivs.unwrap_or_default(),
            evs: StatSet::default(),
            current_hp: None,
            moves: None,
            exp: 0,
            friendship: 70,

        }

    }

}