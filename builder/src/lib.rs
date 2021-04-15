use std::fs::File;
use std::io::Write;

use error::EntryError;

pub mod error;

mod pokemon;
mod moves;

pub fn compile(pokemon_dir: &str, move_dir: &str, save_file: &str, include_audio: bool) -> Result<usize, EntryError> {

    let mut file = File::create(save_file)?;

    let pokemon = pokemon::get_pokemon(pokemon_dir, include_audio)?;
        
    let moves = moves::get_moves(move_dir)?;
    

    let size = file.write(
        &postcard::to_allocvec(
            &firecore_pokedex_lib::serialized::SerializedDex {
                pokemon,
                moves
            }
        )?
    )?;

    Ok(size)

}