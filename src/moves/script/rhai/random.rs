use core::ops::{Deref, DerefMut};

use rand::Rng;
use rhai::INT;

use crate::pokemon::PokemonInstance;

#[derive(Clone)]
pub struct ScriptRandom<R: Rng + Clone + 'static>(*mut R);

impl<R: Rng + Clone + 'static> ScriptRandom<R> {
    pub fn new(random: &mut R) -> Self {
        Self(random as _)
    }
    pub fn crit(&mut self, rate: INT) -> bool {
        PokemonInstance::crit(self.deref_mut(), rate as _)
    }
    pub fn damage_range(&mut self) -> INT {
        PokemonInstance::damage_range(self.deref_mut()) as _
    }
}

impl<R: Rng + Clone + 'static> Deref for ScriptRandom<R> {
    type Target = R;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}

impl<R: Rng + Clone + 'static> DerefMut for ScriptRandom<R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.0 }
    }
}