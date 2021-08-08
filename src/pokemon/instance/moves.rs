use core::hash::Hash;
use hashbrown::HashMap;
use log::error;
use rand::Rng;

use crate::{
    moves::{
        script::MoveEngine,
        usage::{DamageKind, DamageResult, MoveResult, MoveUseType, NoHitResult},
        CriticalRate, Move, MoveCategory, MoveInstance, Power,
    },
    pokemon::{stat::StatStage, Health},
    types::{Effective, PokemonType},
};

impl<'a> super::InitPokemon<'a> {
    // To - do: uses PP on use
    pub fn use_own_move<ID: Eq + Hash, R: Rng + Clone + 'static, E: MoveEngine>(
        &self,
        random: &mut R,
        engine: &mut E,
        move_index: usize,
        targets: HashMap<ID, &Self>,
    ) -> Option<(&Move, HashMap<ID, Vec<MoveResult>>)> {
        let used_move = self
            .moves
            .get(move_index)
            .map(MoveInstance::try_use)
            .flatten()?;

        let targets = targets
            .into_iter()
            .map(|(id, target)| {
                (
                    id,
                    self.use_move_on_target(random, engine, used_move, target),
                )
            })
            .collect();

        Some((used_move, targets))
    }

    pub fn use_move_on_target<R: Rng + Clone + 'static, E: MoveEngine>(
        &self,
        random: &mut R,
        engine: &mut E,
        used_move: &Move,
        target: &Self,
    ) -> Vec<MoveResult> {
        let hit = used_move
            .accuracy
            .map(|accuracy| {
                let hit: u8 = random.gen_range(0..=100);
                hit < accuracy
            })
            .unwrap_or(true);

        match hit {
            false => vec![MoveResult::NoHit(NoHitResult::Miss)],
            true => {
                let mut results = Vec::with_capacity(used_move.usages());
                self.move_usage(
                    random,
                    engine,
                    &mut results,
                    &used_move.usage,
                    used_move,
                    target,
                );
                results
            }
        }
    }

    fn move_usage<R: Rng + Clone + 'static, E: MoveEngine>(
        &self,
        random: &mut R,
        engine: &mut E,
        results: &mut Vec<MoveResult>,
        usage: &Vec<MoveUseType>,
        used_move: &Move,
        target: &Self,
    ) {
        for usage in usage {
            match usage {
                MoveUseType::Damage(kind) => {
                    results.push(
                        match self.damage_kind(
                            random,
                            target,
                            *kind,
                            used_move.category,
                            used_move.pokemon_type,
                            used_move.crit_rate,
                        ) {
                            Some(result) => MoveResult::Damage(result),
                            None => MoveResult::NoHit(NoHitResult::Ineffective),
                        },
                    );
                }
                MoveUseType::Ailment(status, length, chance) => {
                    if target.ailment.is_none() {
                        if random.gen_bool(*chance as f64 / 100.0) {
                            results.push(MoveResult::Status(length.init(*status, random)));
                        }
                    }
                }
                MoveUseType::Drain(kind, percent) => {
                    results.push(
                        match self.damage_kind(
                            random,
                            target,
                            *kind,
                            used_move.category,
                            used_move.pokemon_type,
                            used_move.crit_rate,
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
                    log::error!("to-do: stat stage check");
                    // if target.stages.can_change_stage(&stat) {
                    results.push(MoveResult::StatStage(stat));
                    // }
                }
                // MoveUseType::Linger(..) => {
                // 	results.insert(target.instance, Some(MoveResult::Todo));
                // }
                MoveUseType::Flinch => results.push(MoveResult::Flinch),
                MoveUseType::Chance(usage, chance) => {
                    if random.gen_range(0..=100) < *chance {
                        self.move_usage(random, engine, results, usage, used_move, target);
                    }
                }
                MoveUseType::User(usage) => {
                    // if !results.contains_key(&MoveTargetLocation::User) {
                    self.move_usage(random, engine, results, usage, used_move, self);
                    // }
                }
                MoveUseType::Script(script) => {
                    match engine.execute(script, random, used_move, self, target) {
                        Ok(script_results) => results.extend(script_results),
                        Err(err) => {
                            error!(
                                "Could not execute move script for {} with error {}",
                                used_move.name, err
                            );
                            results.push(MoveResult::NoHit(NoHitResult::Error));
                        }
                    }
                }
                MoveUseType::Todo => {
                    results.push(MoveResult::NoHit(NoHitResult::Todo));
                }
            }
        }
    }

    pub fn damage_kind(
        &self,
        random: &mut impl Rng,
        target: &Self,
        kind: DamageKind,
        category: MoveCategory,
        move_type: PokemonType,
        crit_rate: CriticalRate,
    ) -> Option<DamageResult<Health>> {
        match kind {
            DamageKind::Power(power) => {
                self.move_power_damage_random(random, target, power, category, move_type, crit_rate)
            }
            DamageKind::PercentCurrent(percent) => {
                let effective = target.pokemon.effective(move_type, category);
                (!matches!(effective, Effective::Ineffective)).then(|| DamageResult {
                    damage: (target.hp() as f32 * effective.multiplier() * percent as f32 / 100.0)
                        as Health,
                    effective,
                    crit: false,
                })
            }
            DamageKind::PercentMax(percent) => {
                let effective = target.pokemon.effective(move_type, category);
                (!matches!(effective, Effective::Ineffective)).then(|| DamageResult {
                    damage: (target.max_hp() as f32 * effective.multiplier() * percent as f32
                        / 100.0) as Health,
                    effective,
                    crit: false,
                })
            }
            DamageKind::Constant(damage) => {
                let effective = target.pokemon.effective(move_type, category);
                (!matches!(effective, Effective::Ineffective)).then(|| DamageResult {
                    damage,
                    effective,
                    crit: false,
                })
            }
        }
    }

    pub fn move_power_damage_random(
        &self,
        random: &mut impl Rng,
        target: &Self,
        power: Power,
        category: MoveCategory,
        move_type: PokemonType,
        crit_rate: CriticalRate,
    ) -> Option<DamageResult<Health>> {
        self.move_power_damage(
            target,
            power,
            category,
            move_type,
            Self::crit(random, crit_rate),
            Self::damage_range(random),
        )
    }

    pub fn crit(random: &mut impl Rng, crit_rate: CriticalRate) -> bool {
        random.gen_bool(match crit_rate {
            0 => 0.0625, // 1 / 16
            1 => 0.125,  // 1 / 8
            2 => 0.25,   // 1 / 4
            3 => 1.0 / 3.0,
            _ => 0.5, // rates 4 and above, 1 / 2
        })
    }

    pub fn damage_range(random: &mut impl Rng) -> u8 {
        random.gen_range(85..=100u8)
    }

    pub fn move_power_damage(
        &self,
        target: &Self,
        power: Power,
        category: MoveCategory,
        move_type: PokemonType,
        crit: bool,
        damage_range: u8,
    ) -> Option<DamageResult<Health>> {
        let effective = target.pokemon.effective(move_type, category);
        let (attack, defense) = category.stats();
        let attack = self.stat(attack);
        let defense = target.stat(defense);
        if effective == Effective::Ineffective {
            return None;
        }
        let damage =
            (((((2.0 * self.level as f64 / 5.0 + 2.0).floor() * attack as f64 * power as f64
                / defense as f64)
                .floor()
                / 50.0)
                .floor()
                * effective.multiplier() as f64)
                + 2.0)
                * (damage_range as f64 / 100.0)
                * if self.pokemon.primary_type == move_type {
                    1.5
                } else {
                    1.0
                }
                * if crit { 1.5 } else { 1.0 };
        let damage = damage.min(u16::MAX as f64) as u16;
        Some(DamageResult {
            damage,
            effective,
            crit,
        })
    }
}