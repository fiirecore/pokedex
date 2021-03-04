use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct PokemonCry {

    pub path: String,
    #[serde(skip)]
    pub sound_bytes: Vec<u8>,

}