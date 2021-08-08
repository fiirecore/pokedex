use core::ops::Range;

use rand::Rng;
use serde::{Deserialize, Serialize};

pub type Stat = u8;
pub type Stats = StatSet<Stat>;

mod base;
pub use base::*;

mod full;
pub use full::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum StatType {
    Health,
    Attack,
    Defense,
    SpAttack,
    SpDefense,
    Speed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub struct StatSet<S> {
    pub hp: S,
    pub atk: S,
    pub def: S,
    pub sp_atk: S,
    pub sp_def: S,
    pub speed: S,
}

impl<S: Sized + Copy> StatSet<S> {
    pub fn uniform(stat: S) -> Self {
        Self {
            hp: stat,
            atk: stat,
            def: stat,
            sp_atk: stat,
            sp_def: stat,
            speed: stat,
        }
    }
}

impl<S> StatSet<S> {
    pub fn get(&self, stat: StatType) -> &S {
        match stat {
            StatType::Health => &self.hp,
            StatType::Attack => &self.atk,
            StatType::Defense => &self.def,
            StatType::SpAttack => &self.sp_atk,
            StatType::SpDefense => &self.sp_def,
            StatType::Speed => &self.speed,
        }
    }

    pub fn get_mut(&mut self, stat: StatType) -> &mut S {
        match stat {
            StatType::Health => &mut self.hp,
            StatType::Attack => &mut self.atk,
            StatType::Defense => &mut self.def,
            StatType::SpAttack => &mut self.sp_atk,
            StatType::SpDefense => &mut self.sp_def,
            StatType::Speed => &mut self.speed,
        }
    }
}

impl Stats {
    pub const MAX_EV: Stat = 32;
    pub const MAX_IV: Stat = 252;
    pub const MAX_IVS_TOTAL: u16 = 512;
    pub const EV_RANGE: Range<Stat> = 0..Self::MAX_EV;

    pub fn random(random: &mut impl Rng) -> Self {
        Self {
            hp: random.gen_range(Self::EV_RANGE),
            atk: random.gen_range(Self::EV_RANGE),
            def: random.gen_range(Self::EV_RANGE),
            sp_atk: random.gen_range(Self::EV_RANGE),
            sp_def: random.gen_range(Self::EV_RANGE),
            speed: random.gen_range(Self::EV_RANGE),
        }
    }
}

pub fn default_iv() -> Stats {
    Stats::uniform(15)
}