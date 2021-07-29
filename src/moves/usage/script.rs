use rand::Rng;
use rhai::{exported_module, plugin::*, Dynamic, INT};

pub use rhai::Engine;

use crate::{
    moves::{
        usage::{DamageResult, MoveResult},
        Move, MoveCategory, Power,
    },
    pokemon::{
        instance::PokemonInstance,
        stat::{BaseStat, BattleStatType, StatType},
        Health,
    },
    types::{Effective, PokemonType},
};

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

impl DamageResult<INT> {
    fn damage(&mut self) -> INT {
        self.damage
    }
    fn set_damage(&mut self, damage: INT) {
        self.damage = damage;
    }
    fn effective(&mut self) -> Effective {
        self.effective
    }
}

impl From<DamageResult<Health>> for DamageResult<INT> {
    fn from(result: DamageResult<Health>) -> Self {
        Self {
            damage: result.damage as _,
            effective: result.effective,
            crit: result.crit,
        }
    }
}

impl Into<DamageResult<Health>> for DamageResult<INT> {
    fn into(self) -> DamageResult<Health> {
        DamageResult {
            damage: self.damage as _,
            effective: self.effective,
            crit: self.crit,
        }
    }
}

impl PokemonInstance {
    fn get_damage_rhai(
        user: &mut Self,
        use_type: PokemonType,
        power: INT,
        target_def: INT,
        effective: Effective,
        crit: bool,
        damage_range: INT,
    ) -> DamageResult<INT> {
        user.move_power_damage_stat(
            effective,
            power as Power,
            user.base.get(BattleStatType::Basic(StatType::Attack)),
            target_def as BaseStat,
            user.pokemon.primary_type == use_type,
            crit,
            damage_range as _,
        )
        .map(DamageResult::from)
        .unwrap_or(DamageResult {
            damage: 0,
            effective: Effective::Ineffective,
            crit: false,
        })
    }
    fn effective_rhai(&mut self, pokemon_type: PokemonType, category: MoveCategory) -> Effective {
        self.effective(pokemon_type, category)
    }
    fn defense_rhai(&mut self, category: MoveCategory) -> INT {
        self.base.get(BattleStatType::Basic(category.defense())) as INT
    }

    fn current_hp(&mut self) -> INT {
        self.hp() as INT
    }
    fn primary_type(&mut self) -> PokemonType {
        self.pokemon.primary_type
    }
    // fn get_ref(&mut self) -> &Self {
    //     self
    // }
}

pub fn engine<R: Rng + Clone + 'static>() -> Engine {
    let mut engine = Engine::new_raw();

    engine
        // .register_type_with_name::<PokemonType>("Type")
        // .register_fn("effective", PokemonType::effective)
        // .register_type::<Effective>()
        .register_type_with_name::<ScriptRandom<R>>("Random")
        .register_fn("crit", ScriptRandom::<R>::crit)
        .register_fn("damage_range", ScriptRandom::<R>::damage_range)
        .register_type_with_name::<DamageResult<INT>>("Damage")
        .register_get("damage", DamageResult::damage)
        .register_get("effective", DamageResult::effective)
        .register_set("damage", DamageResult::set_damage)
        .register_type_with_name::<MoveCategory>("Category")
        .register_type_with_name::<PokemonInstance>("Pokemon")
        .register_fn("damage", PokemonInstance::get_damage_rhai)
        .register_fn("effective", PokemonInstance::effective_rhai)
        .register_fn("defense", PokemonInstance::defense_rhai)
        // .register_fn("ref", PokemonInstance::get_ref)
        .register_get("current_hp", PokemonInstance::current_hp)
        .register_get("primary_type", PokemonInstance::primary_type)
        .register_type::<Move>()
        .register_get("category", Move::get_category)
        .register_get("type", Move::get_type)
        .register_get("crit_rate", Move::get_crit_rate)
        // .register_type_with_name::<MoveTargetLocation>("MoveTarget")
        // .register_static_module("MoveTarget", deps::rhai::exported_module!(move_target_instance).into())
        .register_type_with_name::<MoveResult>("MoveResult")
        .register_static_module("MoveResult", exported_module!(move_result).into());

    engine
}

impl Move {
    fn get_category(&mut self) -> MoveCategory {
        self.category
    }
    fn get_type(&mut self) -> PokemonType {
        self.pokemon_type
    }
    fn get_crit_rate(&mut self) -> INT {
        self.crit_rate as INT
    }
}

#[allow(non_snake_case, non_upper_case_globals)]
#[export_module]
mod move_result {
    use rhai::INT;

    use crate::moves::usage::MoveResult;

    use super::DamageResult;

    pub fn Damage(damage: DamageResult<INT>) -> MoveResult {
        MoveResult::Damage(damage.into())
    }
    // pub const fn Status(effect: StatusEffect) -> MoveResult { MoveResult::Status(effect) }
    pub fn Drain(damage: DamageResult<INT>, heal: INT) -> MoveResult {
        MoveResult::Drain(damage.into(), heal as _)
    }
}