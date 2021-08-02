use std::ops::Deref;

use rhai::INT;

use crate::{
    moves::{usage::DamageResult, MoveCategory, Power},
    pokemon::{
        stat::{BaseStat, FullStatType, StatType},
        PokemonInstance,
    },
    types::{Effective, PokemonType},
};

use super::ScriptDamage;

#[derive(Clone)]
pub struct ScriptPokemon(*const PokemonInstance);

impl ScriptPokemon {
    pub fn get_damage(
        &mut self,
        use_type: PokemonType,
        power: INT,
        target_def: INT,
        effective: Effective,
        crit: bool,
        damage_range: INT,
    ) -> ScriptDamage {
        self.move_power_damage_stat(
            effective,
            power as Power,
            self.base.get(FullStatType::Basic(StatType::Attack)),
            target_def as BaseStat,
            self.pokemon.primary_type == use_type,
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
    pub fn effective(&mut self, pokemon_type: PokemonType, category: MoveCategory) -> Effective {
        PokemonInstance::effective(self, pokemon_type, category)
    }
    pub fn defense(&mut self, category: MoveCategory) -> INT {
        self.base.get(FullStatType::Basic(category.defense())) as INT
    }

    pub fn current_hp(&mut self) -> INT {
        self.hp() as INT
    }
    pub fn primary_type(&mut self) -> PokemonType {
        self.pokemon.primary_type
    }
}

impl Deref for ScriptPokemon {
    type Target = PokemonInstance;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
