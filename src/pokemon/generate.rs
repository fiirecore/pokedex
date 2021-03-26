use firecore_pokedex_lib::pokemon::{
    PokemonId,
    Level,
    data::{
        StatSet
    },
};

pub trait GeneratePokemon {

    fn generate(id: PokemonId, min: Level, max: Level, ivs: Option<StatSet>) -> Self;

    fn generate_with_level(id: PokemonId, level: Level, ivs: Option<StatSet>) -> Self where Self: Sized {
        GeneratePokemon::generate(id, level, level, ivs)
    }

}