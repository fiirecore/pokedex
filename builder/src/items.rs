use firecore_pokedex::serialize::SerializedItem;

pub fn get_items<P: AsRef<std::path::Path>>(item_dir: P) -> Vec<SerializedItem> {
    let item_dir = item_dir.as_ref();
    std::fs::read_dir(item_dir)
        .unwrap_or_else(|err| panic!("Could not read item directory at {:?} with error {}", item_dir, err))
            .map(|entry| match entry.map(|entry| entry.path()) {
                Ok(path) => {
                    let data = std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("Could not read item entry at {:?} to string with error {}", path, err));
                    let pokemon_move = toml::from_str(&data).unwrap_or_else(|err| panic!("Could not deserialize item entry at {:?} with error {}", path, err));
                    Some(pokemon_move)
                }
                Err(err) => {
                    eprintln!("Could not read directory item entry with error {}", err);
                    None
                },
            }).flatten().collect()
}