pub extern crate firecore_pokedex as pokedex;

use std::path::Path;

use pokedex::{item::Item, moves::Move, pokemon::Pokemon, BasicDex};

pub mod items;
pub mod moves;
pub mod pokemon;

pub fn compile(
    pokemon: impl AsRef<Path>,
    moves: impl AsRef<Path>,
    items: impl AsRef<Path>,
) -> (BasicDex<Pokemon>, BasicDex<Move>, BasicDex<Item>) {
    // #[cfg(feature = "gen")]
    // gen::gen(pokemon_dir, move_dir)

    println!("Loading pokemon...");
    let pokemon = pokemon::get_pokemon(pokemon);
    println!("Loading moves...");
    let moves = moves::get_moves(moves);
    println!("Loading items...");
    let items = items::get_items(items);

    (pokemon, moves, items)
}
