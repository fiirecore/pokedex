use core::ops::Range;
use serde::{Deserialize, Serialize};

use crate::moves::MoveId;
use crate::pokemon::Level;

mod breeding;
mod training;

pub use breeding::*;
pub use training::*;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum Gender {
    None,
    Male,
    Female,
}

impl Gender {
    pub(crate) const RANGE: Range<u8> = 0..8;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PokedexData {
    pub species: String,
    pub height: u8,
    pub weight: u16,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearnableMove {
    #[serde(rename = "move")]
    pub id: MoveId,
    pub level: Level,
}
