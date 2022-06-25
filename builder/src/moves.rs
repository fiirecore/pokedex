use std::{
    fs::{read_dir, read_to_string},
    path::Path,
};

use pokedex::moves::Move;

pub fn get_moves(moves: impl AsRef<Path>) -> super::Dex<Move> {
    let move_dir = moves.as_ref();

    read_dir(move_dir)
        .unwrap_or_else(|err| {
            panic!(
                "Could not read moves directory at {:?} with error {}",
                move_dir, err
            )
        })
        .flat_map(|entry| match entry.map(|entry| entry.path()) {
            Ok(path) => match path.is_file() {
                true => {
                    let m = ron::from_str::<Move>(&read_to_string(&path).unwrap_or_else(|err| {
                        panic!(
                            "Could not read move file at {:?} to string with error {}",
                            path, err
                        )
                    }))
                    .unwrap_or_else(|err| {
                        panic!("Could not parse move file at {:?} with error {}", path, err)
                    });
                    Some((m.id, m))
                }
                false => None,
            },
            Err(err) => {
                eprintln!("Could not read directory entry with error {}", err);
                None
            }
        })
        .collect()
}
