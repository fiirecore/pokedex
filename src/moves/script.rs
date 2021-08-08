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
        script: &str,
        random: &mut R,
        used_move: &Move,
        user: &InitPokemon<'a>,
        target: &InitPokemon<'a>,
    ) -> Result<Vec<MoveResult>, Self::Error>;
}

pub struct DefaultMoveEngine;

impl MoveEngine for DefaultMoveEngine {
    type Error = MoveScriptError;

    fn execute<'a, R: Rng + Clone + 'static>(
        &mut self,
        _: &str,
        _: &mut R,
        _: &Move,
        _: &InitPokemon<'a>,
        _: &InitPokemon<'a>,
    ) -> Result<Vec<MoveResult>, Self::Error> {
        Err(MoveScriptError)
    }
}

#[derive(Debug)]
pub struct MoveScriptError;

impl Error for MoveScriptError { }

impl core::fmt::Display for MoveScriptError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self, f)
    }
}