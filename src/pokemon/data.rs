use core::ops::Range;
use serde::{Deserialize, Serialize};

use crate::{
    moves::MoveId,
    pokemon::{Level, Experience},
};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Gender {
    Male,
    Female,
}

impl Gender {
    pub(crate) const RANGE: Range<u8> = 0..8;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearnableMove(pub Level, pub MoveId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Training {
    pub base_exp: u16,
    #[serde(default)]
    pub growth_rate: GrowthRate,
    //pub ev_yield: Option<(String, usize)>,
    //pub catch_rate: Option<u8>,
    //pub base_friendship: Option<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Breeding {
    // pub groups: Vec<EggGroup>,
    /// Pokemon gender chance: None = no gender, 0 = 100% female, 7 = 100% male (0-8 scale)
    pub gender: Option<u8>,
    // pub cycles: Option<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GrowthRate {
    Slow,
    Fast,
    Medium,
    MediumSlow,
    FastThenVerySlow,
    SlowThenVeryFast,
}

impl Default for GrowthRate {
    fn default() -> Self {
        Self::MediumSlow
    }
}

impl GrowthRate {
    pub fn max_exp(self, level: Level) -> Experience {
        (match level as i32 {
            0 => 0,
            1 => 1,
            level => match self {
                GrowthRate::Slow => 5 * (level.pow(3) >> 2),
                GrowthRate::Fast => (level.pow(3) << 2) / 5,
                GrowthRate::Medium => level.pow(3),
                GrowthRate::MediumSlow => {
                    (6 * level.pow(3)) / 5 - (15 * level.pow(2)) + (100 * level) - 140
                }
                _ => {
                    (1.2 * level.pow(3) as f32) as i32 - 15 * level.pow(2) as i32
                        + 100 * level as i32
                        - 140
                } // MediumSlow
            },
        }) as Experience
    }
}
