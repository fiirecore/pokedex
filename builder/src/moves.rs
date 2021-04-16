use firecore_pokedex::moves::PokemonMove;

use crate::error::EntryError;

pub fn get_moves<P: AsRef<std::path::Path>>(move_dir: P) -> Result<Vec<PokemonMove>, EntryError> {
    let mut moves = Vec::with_capacity(334);
    for entry in std::fs::read_dir(move_dir.as_ref())? {
        match entry.map(|entry| entry.path()) {
            Ok(path) => {
                let data = std::fs::read_to_string(&path)?;
                let pokemon_move = toml::from_str(&data).map_err(|err| EntryError::ParseError(path.to_string_lossy().to_string(), err))?;
                moves.push(pokemon_move);
            }
            Err(err) => eprintln!("Could not read directory entry with error {}", err),
        }
    }
    Ok(moves)
}