use rand::Rng;
use rhai::INT;

use crate::pokemon::PokemonInstance;

#[derive(Clone)]
pub struct ScriptRandom<R: Rng + Clone + 'static>(R);

impl<R: Rng + Clone + 'static> ScriptRandom<R> {
    pub fn from(random: &R) -> Self {
        Self(random.clone())
    }
    pub fn crit(&mut self, rate: INT) -> bool {
        PokemonInstance::crit(&mut self.0, rate as _)
    }
    pub fn damage_range(&mut self) -> INT {
        PokemonInstance::damage_range(&mut self.0) as _
    }
}