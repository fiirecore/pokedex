use rand::Rng;

use crate::{
    moves::{
        script::MoveEngine,
        usage::{DamageResult, MoveResult},
        Move, MoveCategory,
    },
    pokemon::InitPokemon,
    types::PokemonType,
};

use rhai::{exported_module, plugin::*, Array, Dynamic, Engine, Scope, INT};

mod damage;
mod moves;
mod pokemon;
mod random;

pub use damage::*;
pub use moves::*;
pub use pokemon::*;
pub use random::*;
pub use result::*;

pub struct RhaiMoveEngine {
    engine: Engine,
}

impl RhaiMoveEngine {
    pub fn new<R: Rng + Clone + 'static>() -> Self {
        let mut engine = Engine::new_raw();

        engine
            .register_type_with_name::<ScriptRandom<R>>("Random")
            .register_fn("crit", ScriptRandom::<R>::crit)
            .register_fn("damage_range", ScriptRandom::<R>::damage_range)
            .register_type_with_name::<DamageResult<INT>>("Damage")
            .register_set("damage", ScriptDamage::set_damage)
            .register_get("damage", ScriptDamage::get_damage)
            .register_get("effective", ScriptDamage::effective)
            .register_type_with_name::<ScriptPokemon>("Pokemon")
            .register_fn("damage", ScriptPokemon::get_damage)
            .register_get("current_hp", ScriptPokemon::current_hp)
            .register_type::<ScriptMove>()
            .register_get("category", ScriptMove::get_category)
            .register_get("type", ScriptMove::get_type)
            .register_get("crit_rate", ScriptMove::get_crit_rate)
            .register_type_with_name::<MoveCategory>("Category")
            .register_type_with_name::<PokemonType>("Type")
            .register_type_with_name::<MoveResult>("MoveResult")
            .register_static_module("MoveResult", exported_module!(result).into());

        Self { engine }
    }
}

impl MoveEngine for RhaiMoveEngine {
    type Error = Box<EvalAltResult>;

    fn execute<'a, R: Rng + Clone + 'static>(
        &mut self,
        script: &str,
        random: &mut R,
        used_move: &Move,
        user: &InitPokemon<'a>,
        target: &InitPokemon<'a>,
    ) -> Result<Vec<MoveResult>, Self::Error> {
        let mut scope = Scope::new();
        scope.push("random", ScriptRandom::new(random));
        scope.push("move", ScriptMove::new(used_move));
        scope.push("user", ScriptPokemon::new(user));
        scope.push("target", ScriptPokemon::new(target));

        Ok(self
            .engine
            .eval_with_scope::<Array>(&mut scope, script)?
            .into_iter()
            .flat_map(Dynamic::try_cast::<MoveResult>)
            .collect())
    }
}

#[allow(non_snake_case, non_upper_case_globals)]
#[export_module]
mod result {
    use rhai::INT;

    use crate::moves::usage::MoveResult;

    use super::ScriptDamage;

    pub fn Damage(damage: ScriptDamage) -> MoveResult {
        MoveResult::Damage(damage.into())
    }
    // pub const fn Status(effect: StatusEffect) -> MoveResult { MoveResult::Status(effect) }
    pub fn Drain(damage: ScriptDamage, heal: INT) -> MoveResult {
        MoveResult::Drain(damage.into(), heal as _)
    }
}
