use firecore_pokedex_lib::moves::PokemonMove;

use crate::error::EntryError;

pub fn get_moves(move_dir: &str) -> Result<Vec<PokemonMove>, EntryError> {
    let mut moves = Vec::with_capacity(334);
    for entry in std::fs::read_dir(move_dir)? {
        let path = entry?.path();
        let data = std::fs::read_to_string(&path)?;
        let pokemon_move = toml::from_str(&data).map_err(|err| EntryError::ParseError(path.to_string_lossy().to_string(), err))?;
        moves.push(pokemon_move);
    }
    Ok(moves)
}