use firecore_pokedex::moves::PokemonMove;

pub fn get_moves<P: AsRef<std::path::Path>>(move_dir: P) -> Vec<PokemonMove> {
    let move_dir = move_dir.as_ref();
    std::fs::read_dir(move_dir).unwrap_or_else(|err| panic!("Could not read moves directory at {:?} with error {}", move_dir, err))
        .map(|entry| match entry.map(|entry| entry.path()) {
            Ok(path) => {
                let data = std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("Could not read move at {:?} to string with error {}", path, err));
                let pokemon_move = toml::from_str(&data).unwrap_or_else(|err| panic!("Could not deserialize move at {:?} with error {}", path, err));
                Some(pokemon_move)
            }
            Err(err) => {
                eprintln!("Could not read directory entry with error {}", err);
                None
            },
        }).flatten().collect()
}