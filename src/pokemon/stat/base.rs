use serde::{Deserialize, Serialize};

use crate::pokemon::{Level, Pokemon};

use super::{FullStatSet, FullStatType, Stat, StatSet, Stats};

pub type BaseStat = u16;
pub type Stage = i8;
pub type BaseStatSet = StatSet<BaseStat>;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct StatStages {
    pub stages: FullStatSet<Stage>,
    pub accuracy: Stage,
    pub evasion: Stage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct StatStage {
    pub stat: FullStatType,
    pub stage: Stage,
}

impl StatStages {
    pub fn get(&self, stats: &BaseStatSet, stat: FullStatType) -> BaseStat {
        StatSet::mult(
            match stat {
                FullStatType::Basic(stat) => *stats.get(stat),
                FullStatType::Accuracy => Self::stage_temp(self.accuracy),
                FullStatType::Evasion => Self::stage_temp(self.evasion),
            },
            *self.stages.get(stat),
        )
    }

    fn stage_temp(stage: Stage) -> BaseStat {
        (stage * 13 + 100) as _
    }

    pub fn can_change_stage(&self, stat: &StatStage) -> bool {
        self.stages.get(stat.stat).abs() + stat.stage < 6
    }

    pub fn change_stage(&mut self, stat: StatStage) {
        *self.stages.get_mut(stat.stat) += stat.stage;
    }
}

impl Default for StatStages {
    fn default() -> Self {
        Self {
            stages: Default::default(),
            accuracy: 100,
            evasion: 100,
        }
    }
}

impl BaseStatSet {
    pub fn new(pokemon: &Pokemon, ivs: &Stats, evs: &Stats, level: Level) -> Self {
        Self {
            hp: Self::hp(pokemon.base.hp, ivs.hp, evs.hp, level),
            atk: Self::stat(pokemon.base.atk, ivs.atk, evs.atk, level),
            def: Self::stat(pokemon.base.def, ivs.def, evs.def, level),
            sp_atk: Self::stat(pokemon.base.sp_atk, ivs.sp_atk, evs.sp_atk, level),
            sp_def: Self::stat(pokemon.base.sp_def, ivs.sp_def, evs.sp_def, level),
            speed: Self::stat(pokemon.base.speed, ivs.speed, evs.speed, level),
        }
    }

    pub fn stat(base: Stat, iv: Stat, ev: Stat, level: Level) -> BaseStat {
        //add item check
        let nature = 1.0;
        (((2.0 * base as f32 + iv as f32 + ev as f32) * level as f32 / 100.0 + 5.0).floor()
            * nature)
            .floor() as BaseStat
    }

    pub fn hp(base: Stat, iv: Stat, ev: Stat, level: Level) -> BaseStat {
        ((2.0 * base as f64 + iv as f64 + ev as f64) * level as f64 / 100.0 + level as f64 + 10.0)
            .floor() as BaseStat
    }
}

impl StatSet<Stage> {
    pub fn mult(base: BaseStat, stage: Stage) -> BaseStat {
        base * (2.max(2 + stage) as BaseStat) / (2.max(2 - stage) as BaseStat)
    }
}
