use std::{
    fs::{read_dir, read_to_string},
    path::Path,
};

use pokedex::pokemon::Pokemon;

pub fn get_pokemon(path: impl AsRef<Path>) -> Vec<Pokemon> {
    let path = path.as_ref();

    let readdir = read_dir(path).unwrap_or_else(|err| {
        panic!(
            "Could not read pokemon directory at {:?} with error {}",
            path, err
        )
    });

    readdir
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .map(|file| {
            ron::from_str::<Pokemon>(&read_to_string(&file).unwrap_or_else(|err| {
                panic!(
                    "Could not read pokemon file at {:?} to string with error {}",
                    file, err
                )
            }))
            .unwrap_or_else(|err| {
                panic!(
                    "Could not parse pokemon file at {:?} with error {}",
                    file, err
                )
            })
        })
        .collect()
}
