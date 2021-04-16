use std::fs::File;
use std::fs::read_dir;
use std::io::Read;
use std::path::PathBuf;

use firecore_pokedex::pokemon::Pokemon;
use firecore_pokedex::serialize::SerializedPokemon;

use crate::error::EntryError;

pub fn get_pokemon<P: AsRef<std::path::Path>>(pokemon_dir: P, include_audio: bool) -> Result<Vec<SerializedPokemon>, EntryError> {
    let mut pokemon = Vec::new();

    for entry in read_dir(pokemon_dir.as_ref())? {
        match entry.map(|entry| entry.path()) {
            Ok(dir) => {
                if dir.is_dir() {
                    let pokemon_entry = find_entry_file(&dir)?;
                    let mut front_png = Vec::new();
                    let mut back_png = Vec::new();
                    let mut icon_png = Vec::new();
                    File::open(dir.join("normal_front.png"))?.read_to_end(&mut front_png)?;
                    File::open(dir.join("normal_back.png"))?.read_to_end(&mut back_png)?;
                    File::open(dir.join("icon.png"))?.read_to_end(&mut icon_png)?;
                    let cry_ogg = {
                        if include_audio {
                            if let Ok(mut cry_file) = File::open(dir.join("cry.ogg")) {
                                let mut cry_ogg = Vec::new();
                                cry_file.read_to_end(&mut cry_ogg)?;
                                cry_ogg
                            } else {
                                Vec::new()
                            }
                        } else {
                            Vec::new()
                        }
                    };
        
                    pokemon.push(SerializedPokemon {
                        pokemon: pokemon_entry,
                        cry_ogg,
                        front_png,
                        back_png,
                        icon_png,
                    });
        
                }
            }
            Err(err) => eprintln!("Could not read directory entry with error {}", err),
        }
    }

    println!("Loaded {} pokemon.", pokemon.len());

    Ok(pokemon)
}

fn find_entry_file(dir_path: &PathBuf) -> Result<Pokemon, EntryError> {
    for file_entry in read_dir(&dir_path)? {
        let file = file_entry?.path();
        if let Some(ext) = file.extension() {
            if ext == std::ffi::OsString::from("toml") {
                let data = std::fs::read_to_string(&file)?;
                return Ok(toml::from_str(&data).map_err(|err| EntryError::ParseError(file.to_string_lossy().to_string(), err))?);
            }
        }
    }
    Err(EntryError::NoEntry)
}