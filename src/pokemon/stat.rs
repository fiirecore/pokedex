use core::ops::Range;

use rand::Rng;
use serde::{Deserialize, Serialize};

/// A stat amount for a Pokemon.
/// Used in a Pokemon's IVs and EVs.
pub type Stat = u8;
/// BaseStats are [Stat]s, but have a larger range.
pub type BaseStat = u16;

/// A [StatSet] of [Stat]s.
pub type Stats = StatSet<Stat>;

/// The type of [Stat].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum StatType {
    Health,
    Attack,
    Defense,
    /// Special Attack
    SpAttack,
    /// Special Defense
    SpDefense,
    /// The speed of a pokemon decides if it moves before another. If a pokemon's speed is higher than another's, it goes first.
    Speed,
}

/// A set of all [StatType]s, with a provided stat generic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct StatSet<S> {
    pub hp: S,
    pub atk: S,
    pub def: S,
    pub sp_atk: S,
    pub sp_def: S,
    pub speed: S,
}

impl<S> StatSet<S> {
    /// Create a [StatSet] with one type of provided stat.
    pub fn uniform(stat: S) -> Self
    where
        S: Copy,
    {
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

impl Stats {
    /// The maximum value of an IV stat.
    pub const MAX_IV: Stat = 32;
    /// The maximum value of an EV stat.
    pub const MAX_EV: Stat = 252;

    /// The maximum amount of all EVs added up in a [StatSet].
    pub const MAX_EVS_TOTAL: u16 = 512;

    /// The numerical range of an IV stat.
    pub const IV_RANGE: Range<Stat> = 0..Self::MAX_IV;
    /// The numerical range of an EV stat.
    pub const EV_RANGE: Range<Stat> = 0..Self::MAX_EV;

    /// Generate a random [Stat] in the IV_RANGE
    pub fn random_iv(random: &mut impl Rng) -> Self {
        Self {
            hp: random.gen_range(Self::IV_RANGE),
            atk: random.gen_range(Self::IV_RANGE),
            def: random.gen_range(Self::IV_RANGE),
            sp_atk: random.gen_range(Self::IV_RANGE),
            sp_def: random.gen_range(Self::IV_RANGE),
            speed: random.gen_range(Self::IV_RANGE),
        }
    }

    /// Get a [Stat] from a [StatType].
    pub fn get(&self, stat: StatType) -> Stat {
        match stat {
            StatType::Health => self.hp,
            StatType::Attack => self.atk,
            StatType::Defense => self.def,
            StatType::SpAttack => self.sp_atk,
            StatType::SpDefense => self.sp_def,
            StatType::Speed => self.speed,
        }
    }

    /// Increment a [StatType] by an amount.
    /// This function is for a pokemon's EV stats.
    pub fn increment_ev(&mut self, stat: StatType, by: Stat) {
        if self.total() + by < Self::MAX_EV {
            let stat = match stat {
                StatType::Health => &mut self.hp,
                StatType::Attack => &mut self.atk,
                StatType::Defense => &mut self.def,
                StatType::SpAttack => &mut self.sp_atk,
                StatType::SpDefense => &mut self.sp_def,
                StatType::Speed => &mut self.speed,
            };

            *stat = stat.saturating_add(by);
        }
    }

    /// The total count of stat values in a [StatSet].
    pub fn total(&self) -> Stat {
        self.hp
            .saturating_add(self.atk)
            .saturating_add(self.def)
            .saturating_add(self.sp_atk)
            .saturating_add(self.sp_def)
            .saturating_add(self.speed)
    }

    /// Get the default IV [StatSet].
    pub fn default_iv() -> Self {
        Self::uniform(15)
    }
}
