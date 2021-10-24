//! Status effects implementation
//! 
//! This module is very incomplete and likely to change

use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum Ailment {
    Paralysis,
    Sleep,
    Freeze,
    Burn, 
    Poison,
}

pub type AilmentEffect = AilmentInstance<AilmentLength>;
pub type LiveAilment = AilmentInstance<Remaining>;

pub type Remaining = Option<u8>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct AilmentInstance<T> {
    pub ailment: Ailment,
    pub turns: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum AilmentLength {
    Permanent,
    Temporary(u8, u8),
}


// impl AilmentEffect {
//     pub fn init(&self, random: &mut impl Rng) -> LiveAilment {
//         LiveAilment {
//             ailment: self.ailment,
//             turns: self.turns.init(random),
//         }
//     }
// }

impl AilmentLength {

    pub fn init(&self, ailment: Ailment, random: &mut impl Rng) -> LiveAilment {
        LiveAilment {
            ailment,
            turns: self.get(random),
        }
    }

    pub fn get(self, random: &mut impl Rng) -> Remaining {
        match self {
            AilmentLength::Temporary(min, max) => Some(random.gen_range(min..=max)),
            AilmentLength::Permanent => None,
        }
    }
}