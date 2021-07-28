use rand::Rng;
use rhai::{Array, Engine, Scope};

use crate::{
    moves::{
        instance::MoveInstance,
        target::MoveTargetLocation,
        usage::{
            DamageKind, DamageResult, MoveResult, MoveResults, MoveUseType, NoHitResult,
            PokemonTarget, TurnResult,
        },
        CriticalRate, Move, MoveCategory, MoveRef, Power,
    },
    pokemon::{
        instance::PokemonInstance,
        stat::{BaseStat, BattleStatType, StatStage},
        Health,
    },
    types::{Effective, PokemonType},
};

impl PokemonInstance {
    pub fn replace_move(&mut self, index: usize, move_ref: MoveRef) {
        self.moves[index] = MoveInstance::new(move_ref);
    }

    // To - do: uses PP on use
    pub fn use_own_move(
        &self,
        random: &mut impl Rng,
        engine: &Engine,
        move_index: usize,
        targets: Vec<PokemonTarget>,
    ) -> TurnResult {
        let pokemon_move = self
            .moves
            .get(move_index)
            .map(|i| i.move_ref)
            .unwrap_or_else(|| {
                panic!(
                    "Could not get move at index {} for pokemon {}",
                    move_index,
                    self.name()
                )
            });
        let mut results = MoveResults::new();

        for target in targets {
            self.use_move_on_target(random, engine, &mut results, &pokemon_move, target);
        }

        TurnResult {
            pokemon_move,
            results,
        }
        // check if target is in move target enum
    }

    pub fn use_move_on_target(
        &self,
        random: &mut impl Rng,
        engine: &Engine,
        results: &mut MoveResults,
        pokemon_move: &Move,
        target: PokemonTarget,
    ) {
        let hit = pokemon_move
            .accuracy
            .map(|accuracy| {
                let hit: u8 = random.gen_range(0..=100);
                hit < accuracy
            })
            .unwrap_or(true);

        if hit {
            self.usage(
                results,
                random,
                engine,
                pokemon_move,
                target,
                &pokemon_move.usage,
            );
        } else {
            results.insert(target.active, vec![MoveResult::NoHit(NoHitResult::Miss)]);
        }
    }

    fn usage(
        &self,
        results: &mut MoveResults,
        random: &mut impl Rng,
        engine: &Engine,
        pokemon_move: &Move,
        target: PokemonTarget,
        usage: &Vec<MoveUseType>,
    ) {
        if !results.contains_key(&target.active) {
            results.insert(target.active, Vec::with_capacity(usage.len()));
        }
        for usage in usage {
            let move_results = results.get_mut(&target.active).unwrap();
            match usage {
                MoveUseType::Damage(kind) => {
                    move_results.push(
                        match self.damage_kind(
                            random,
                            *kind,
                            pokemon_move.category,
                            pokemon_move.pokemon_type,
                            pokemon_move.crit_rate,
                            &target.pokemon,
                        ) {
                            Some(result) => MoveResult::Damage(result),
                            None => MoveResult::NoHit(NoHitResult::Ineffective),
                        },
                    );
                }
                MoveUseType::Status(status, range, chance) => {
                    if target.pokemon.can_afflict_status() {
                        if random.gen_bool(*chance as f64 / 100.0) {
                            move_results.push(MoveResult::Status(range.init(*status, random)));
                        }
                    }
                }
                MoveUseType::Drain(kind, percent) => {
                    move_results.push(
                        match self.damage_kind(
                            random,
                            *kind,
                            pokemon_move.category,
                            pokemon_move.pokemon_type,
                            pokemon_move.crit_rate,
                            &target.pokemon,
                        ) {
                            Some(result) => {
                                let heal = (result.damage as f32 * *percent as f32 / 100.0) as i16;
                                MoveResult::Drain(result, heal)
                            }
                            None => MoveResult::NoHit(NoHitResult::Ineffective),
                        },
                    );
                }
                MoveUseType::StatStage(stat, stage) => {
                    let stat = StatStage {
                        stat: *stat,
                        stage: *stage,
                    };
                    if target.pokemon.base.can_change_stage(&stat) {
                        move_results.push(MoveResult::StatStage(stat));
                    }
                }
                // MoveUseType::Linger(..) => {
                // 	results.insert(target.instance, Some(MoveResult::Todo));
                // }
                MoveUseType::Flinch => move_results.push(MoveResult::Flinch),
                MoveUseType::Chance(usage, chance) => {
                    if random.gen_range(0..=100) < *chance {
                        self.usage(results, random, engine, pokemon_move, target, usage);
                    }
                }
                MoveUseType::User(usage) => {
                    if !results.contains_key(&MoveTargetLocation::User) {
                        self.usage(
                            results,
                            random,
                            engine,
                            pokemon_move,
                            PokemonTarget {
                                pokemon: self,
                                active: MoveTargetLocation::User,
                            },
                            usage,
                        );
                    }
                }
                MoveUseType::Script(script) => {
                    let mut scope = Scope::new();
                    scope.push("move", pokemon_move.clone());
                    scope.push("user", self.clone());
                    scope.push("target", target.pokemon.clone());
                    // scope.push("target_instance", target.instance.clone());

                    match engine.eval_with_scope::<Array>(&mut scope, script) {
                        Ok(hits) => {
                            for hit in hits {
                                match hit.try_cast::<MoveResult>() {
                                    Some(hit) => {
                                        results.get_mut(&target.active).unwrap().push(hit);
                                    }
                                    None => panic!(
                                        "Could not get hit result from returned array for move {}",
                                        pokemon_move
                                    ),
                                }
                            }
                        }
                        Err(err) => panic!("{}", err),
                    }
                }
                MoveUseType::Todo => {
                    move_results.push(MoveResult::NoHit(NoHitResult::Todo));
                }
            }
        }
    }

    pub fn damage_kind(
        &self,
        random: &mut impl Rng,
        kind: DamageKind,
        category: MoveCategory,
        pokemon_type: PokemonType,
        crit_rate: CriticalRate,
        target: &PokemonInstance,
    ) -> Option<DamageResult<Health>> {
        match kind {
            DamageKind::Power(power) => {
                self.move_power_damage(random, target, power, category, pokemon_type, crit_rate)
            }
            DamageKind::PercentCurrent(percent) => {
                let effective = target.effective(pokemon_type, category);
                (!matches!(effective, Effective::Ineffective)).then(|| DamageResult {
                    damage: (target.hp() as f32 * effective.multiplier() * percent as f32 / 100.0)
                        as Health,
                    effective,
                    crit: false,
                })
            }
            DamageKind::PercentMax(percent) => {
                let effective = target.effective(pokemon_type, category);
                (!matches!(effective, Effective::Ineffective)).then(|| DamageResult {
                    damage: (target.max_hp() as f32 * effective.multiplier() * percent as f32
                        / 100.0) as Health,
                    effective,
                    crit: false,
                })
            }
            DamageKind::Constant(damage) => {
                let effective = target.effective(pokemon_type, category);
                (!matches!(effective, Effective::Ineffective)).then(|| DamageResult {
                    damage,
                    effective,
                    crit: false,
                })
            }
        }
    }

    pub fn move_power_damage(
        &self,
        random: &mut impl Rng,
        target: &PokemonInstance,
        power: Power,
        category: MoveCategory,
        use_type: PokemonType,
        crit_rate: CriticalRate,
    ) -> Option<DamageResult<Health>> {
        let effective = target.effective(use_type, category);
        let (attack, defense) = category.stats();
        let (attack, defense) = (
            self.base.get(BattleStatType::Basic(attack)),
            target.base.get(BattleStatType::Basic(defense)),
        );
        self.move_power_damage_stat(
            random,
            effective,
            power,
            attack,
            defense,
            self.pokemon.primary_type == use_type,
            crit_rate,
        )
    }

    pub fn move_power_damage_stat(
        &self,
        random: &mut impl Rng,
        effective: Effective,
        power: Power,
        attack: BaseStat,
        defense: BaseStat,
        same_type_as_user: bool,
        crit_rate: CriticalRate,
    ) -> Option<DamageResult<Health>> {
        if effective == Effective::Ineffective {
            return None;
        }
        let crit = random.gen_bool(match crit_rate {
            0 => 0.0625, // 1 / 16
            1 => 0.125,  // 1 / 8
            2 => 0.25,   // 1 / 4
            3 => 1.0 / 3.0,
            _ => 0.5, // rates 4 and above, 1 / 2
        });
        let damage =
            (((((2.0 * self.level as f64 / 5.0 + 2.0).floor() * attack as f64 * power as f64
                / defense as f64)
                .floor()
                / 50.0)
                .floor()
                * effective.multiplier() as f64)
                + 2.0)
                * (random.gen_range(85..=100u8) as f64 / 100.0)
                * if same_type_as_user { 1.5 } else { 1.0 }
                * if crit { 1.5 } else { 1.0 };
        let damage = damage.min(u16::MAX as f64) as u16;
        Some(DamageResult {
            damage,
            effective,
            crit,
        })
    }
}
