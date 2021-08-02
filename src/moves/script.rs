use rand::Rng;
use std::error::Error;

use crate::pokemon::PokemonInstance;

use super::{usage::MoveResult, Move};

#[cfg(feature = "rhai")]
pub mod rhai;

pub trait MoveEngine {
    type Error: Error;

    fn execute<R: Rng + Clone + 'static>(
        &mut self,
        script: &str,
        random: &mut R,
        used_move: &Move,
        user: &PokemonInstance,
        target: &PokemonInstance,
    ) -> Result<Vec<MoveResult>, Self::Error>;
}

pub struct DefaultMoveEngine;

impl MoveEngine for DefaultMoveEngine {
    type Error = MoveScriptError;

    fn execute<R: Rng + Clone + 'static>(
        &mut self,
        _: &str,
        _: &mut R,
        _: &Move,
        _: &PokemonInstance,
        _: &PokemonInstance,
    ) -> Result<Vec<MoveResult>, Self::Error> {
        Err(MoveScriptError)
    }
}

#[derive(Debug)]
pub struct MoveScriptError;

impl Error for MoveScriptError { }

impl core::fmt::Display for MoveScriptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        core::fmt::Debug::fmt(&self, f)
    }
}