use std::ops::Deref;

use rhai::INT;

use crate::{
    moves::{usage::DamageResult, MoveCategory},
    pokemon::{
        InitPokemon,
    },
    types::{Effective, PokemonType},
};

use super::ScriptDamage;

#[derive(Clone, Copy)]
pub struct ScriptPokemon(*const InitPokemon<'static>);

impl ScriptPokemon {

    pub fn new<'a>(pokemon: &InitPokemon<'a>) -> Self {
        let p = pokemon as *const InitPokemon<'a>;
        let p = unsafe { core::mem::transmute::<*const InitPokemon<'a>, *const InitPokemon<'static>>(p) };
        Self(p)
    }

    pub fn get_damage(
        &mut self,
        target: ScriptPokemon,
        power: INT,
        category: MoveCategory,
        move_type: PokemonType,
        crit: bool,
        damage_range: INT,
    ) -> ScriptDamage {
        self.move_power_damage(
            &target,
            power as _,
            category,
            move_type,
            crit,
            damage_range as _,
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

impl Deref for ScriptPokemon {
    type Target = InitPokemon<'static>;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
