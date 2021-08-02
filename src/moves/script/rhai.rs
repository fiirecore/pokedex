use rand::Rng;

use crate::{
    moves::{
        script::MoveEngine,
        usage::{DamageResult, MoveResult},
        Move, MoveCategory,
    },
    pokemon::PokemonInstance,
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
            // .register_type_with_name::<PokemonType>("Type")
            // .register_fn("effective", PokemonType::effective)
            // .register_type::<Effective>()
            .register_type_with_name::<ScriptRandom<R>>("Random")
            .register_fn("crit", ScriptRandom::<R>::crit)
            .register_fn("damage_range", ScriptRandom::<R>::damage_range)
            .register_type_with_name::<DamageResult<INT>>("Damage")
            .register_set("damage", ScriptDamage::set_damage)
            .register_get("damage", ScriptDamage::get_damage)
            .register_get("effective", ScriptDamage::effective)
            .register_type_with_name::<MoveCategory>("Category")
            .register_type_with_name::<ScriptPokemon>("Pokemon")
            .register_fn("damage", ScriptPokemon::get_damage)
            .register_fn("effective", ScriptPokemon::effective)
            .register_fn("defense", ScriptPokemon::defense)
            .register_get("current_hp", ScriptPokemon::current_hp)
            .register_get("primary_type", ScriptPokemon::primary_type)
            .register_type::<ScriptMove>()
            .register_get("category", ScriptMove::get_category)
            .register_get("type", ScriptMove::get_type)
            .register_get("crit_rate", ScriptMove::get_crit_rate)
            // .register_type_with_name::<MoveTargetLocation>("MoveTarget")
            // .register_static_module("MoveTarget", deps::rhai::exported_module!(move_target_instance).into())
            .register_type_with_name::<MoveResult>("MoveResult")
            .register_static_module("MoveResult", exported_module!(result).into());

        Self { engine }
    }
}

impl MoveEngine for RhaiMoveEngine {
    type Error = Box<EvalAltResult>;

    fn execute<R: Rng + Clone + 'static>(
        &mut self,
        script: &str,
        random: &mut R,
        used_move: &Move,
        user: &PokemonInstance,
        target: &PokemonInstance,
    ) -> Result<Vec<MoveResult>, Self::Error> {
        let mut scope = Scope::new();
        scope.push("random", ScriptRandom::from(random));
        scope.push("move", used_move.clone());
        scope.push("user", user.clone());
        scope.push("target", target.clone());

        let results = self.engine.eval_with_scope::<Array>(&mut scope, script)?;
        let results = results
            .into_iter()
            .flat_map(Dynamic::try_cast::<MoveResult>)
            .collect();
        Ok(results)
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
