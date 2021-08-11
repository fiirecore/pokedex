use rand::Rng;
use std::error::Error;

use crate::pokemon::InitPokemon;

use super::{usage::MoveResult, Move};

#[cfg(feature = "rhai")]
pub mod rhai;

pub trait MoveEngine {
    type Error: Error;

    fn execute<'a, R: Rng + Clone + 'static>(
        &mut self,
        random: &mut R,
        used_move: &Move,
        user: &InitPokemon<'a>,
        target: &InitPokemon<'a>,
        is_user: bool,
    ) -> Result<Vec<MoveResult>, Self::Error>;
}