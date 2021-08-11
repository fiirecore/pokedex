use core::{ops::{Deref, DerefMut}, marker::PhantomData};

use rand::Rng;
use rhai::INT;

use crate::{
    moves::{usage::DamageResult, MoveCategory},
    pokemon::{
        InitPokemon,
    },
    types::{Effective, PokemonType},
};

use super::{ScriptDamage, ScriptRandom};

#[derive(Clone, Copy)]
pub struct ScriptPokemon<R: Rng + Clone + 'static>(*const InitPokemon<'static>, PhantomData<R>);

impl<R: Rng + Clone + 'static> ScriptPokemon<R> {

    pub fn new<'a>(pokemon: &InitPokemon<'a>) -> Self {
        let p = pokemon as *const InitPokemon<'a>;
        let p = unsafe { core::mem::transmute::<*const InitPokemon<'a>, *const InitPokemon<'static>>(p) };
        Self(p, PhantomData)
    }

    pub fn get_damage(
        &mut self,
        random: ScriptRandom<R>,
        target: ScriptPokemon<R>,
        power: INT,
        category: MoveCategory,
        move_type: PokemonType,
        crit_rate: INT,
    ) -> ScriptDamage {
        let mut random = random;
        self.move_power_damage_random(
            random.deref_mut(),
            &target,
            power as _,
            category,
            move_type,
            crit_rate as _,
        )
        .map(ScriptDamage::from)
        .unwrap_or(
            DamageResult {
                damage: 0,
                effective: Effective::Ineffective,
                crit: false,
            }
            .into(),
        )
    }
    pub fn current_hp(&mut self) -> INT {
        self.hp() as INT
    }

}

impl<R: Rng + Clone + 'static> Deref for ScriptPokemon<R> {
    type Target = InitPokemon<'static>;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
