use core::{
    iter::Sum,
    ops::{Index, IndexMut, Range},
};

use enum_map::{EnumArray, EnumMap};
use rand::{
    distributions::uniform::{SampleRange, SampleUniform},
    Rng,
};
use serde::{Deserialize, Serialize};

#[macro_export]
macro_rules! stat_set {
    {$($t:tt)*} => {
        StatSet($crate::pokemon::stat::enum_map! { $($t)* })
    };
}

pub use enum_map::{enum_map, Enum};

/// A stat amount for a Pokemon.
/// Used in a Pokemon's IVs and EVs.
pub type Stat = u8;
/// BaseStats are [Stat]s, but have a larger range.
pub type BaseStat = u16;

/// A [StatSet] of [Stat]s.
pub type Stats = StatSet<StatType, Stat>;

/// The type of [Stat].
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Enum, Serialize, Deserialize,
)]
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
#[derive(Debug, PartialEq, Eq)]
pub struct StatSet<K: EnumArray<S>, S>(pub EnumMap<K, S>);

impl<K: EnumArray<S>, S> StatSet<K, S> {
    /// Create a [StatSet] with one type of provided stat.
    pub fn uniform(stat: S) -> Self
    where
        S: Default + Clone,
    {
        // epic fail
        let mut map = EnumMap::<K, S>::default();
        for s in map.values_mut() {
            *s = stat.clone();
        }
        Self(map)
    }

    /// Get a random [StatSet]
    pub fn random<R: SampleRange<S> + Clone>(random: &mut impl Rng, range: R) -> Self
    where
        S: Default + SampleUniform,
    {
        let mut map = EnumMap::<K, S>::default();
        for s in map.values_mut() {
            *s = random.gen_range(range.clone());
        }
        Self(map)
    }

    /// The total count of stat values in a [StatSet].
    pub fn total(&self) -> S
    where
        S: Sum + Clone,
    {
        self.0.values().cloned().sum()
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
        Self::random(random, Self::IV_RANGE)
    }

    /// Increment a [StatType] by an amount.
    /// This function is for a pokemon's EV stats.
    pub fn increment_ev(&mut self, stat: StatType, by: Stat) {
        if self.total() + by < Self::MAX_EV {
            let stat = &mut self.0[stat];

            *stat = stat.saturating_add(by);
        }
    }

    /// Get the default IV [StatSet].
    pub fn default_iv() -> Self {
        Self::uniform(15)
    }
}

impl<K: EnumArray<S>, S> Index<K> for StatSet<K, S> {
    type Output = S;

    fn index(&self, key: K) -> &S {
        &self.0[key]
    }
}

impl<K: EnumArray<S>, S> IndexMut<K> for StatSet<K, S> {
    fn index_mut(&mut self, key: K) -> &mut S {
        &mut self.0[key]
    }
}

impl<K: EnumArray<S>, S: Default> Default for StatSet<K, S> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<K: EnumArray<S> + Copy, S: Copy> Copy for StatSet<K, S> where K::Array: Copy {}

impl<K: EnumArray<S> + Clone, S: Clone> Clone for StatSet<K, S>
where
    K::Array: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<K: EnumArray<S> + Serialize, S: Serialize> Serialize for StatSet<K, S> {
    fn serialize<SER: serde::Serializer>(&self, serializer: SER) -> Result<SER::Ok, SER::Error> {
        EnumMap::serialize(&self.0, serializer)
    }
}

impl<'de, K: EnumArray<S> + EnumArray<Option<S>> + Deserialize<'de>, S: Deserialize<'de>>
    Deserialize<'de> for StatSet<K, S>
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        EnumMap::deserialize(deserializer).map(Self)
    }
}
