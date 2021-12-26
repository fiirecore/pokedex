use serde::{Deserialize, Serialize};

use super::stat::StatType;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum Nature {
    Adamant,
    Bashful,
    Bold,
    Brave,
    Calm,
    Careful,
    Docile,
    Gentle,
    Hardy,
    Hasty,
    Impish,
    Jolly,
    Lax,
    Lonely,
    Mild,
    Modest,
    Naive,
    Naughty,
    Quiet,
    Quirky,
    Rash,
    Relaxed,
    Sassy,
    Serious,
    Timid,
}

impl Nature {
    pub const COUNT: usize = 25;

    pub const fn increases(&self) -> Option<StatType> {
        match self {
            Nature::Adamant => Some(StatType::Attack),
            Nature::Bashful => None,
            Nature::Bold => Some(StatType::Defense),
            Nature::Brave => Some(StatType::Attack),
            Nature::Calm => Some(StatType::SpDefense),
            Nature::Careful => Some(StatType::SpDefense),
            Nature::Docile => None,
            Nature::Gentle => Some(StatType::SpDefense),
            Nature::Hardy => None,
            Nature::Hasty => Some(StatType::Speed),
            Nature::Impish => Some(StatType::Defense),
            Nature::Jolly => Some(StatType::Speed),
            Nature::Lax => Some(StatType::Defense),
            Nature::Lonely => Some(StatType::Attack),
            Nature::Mild => Some(StatType::SpAttack),
            Nature::Modest => Some(StatType::SpAttack),
            Nature::Naive => Some(StatType::Speed),
            Nature::Naughty => Some(StatType::Attack),
            Nature::Quiet => Some(StatType::SpAttack),
            Nature::Quirky => None,
            Nature::Rash => Some(StatType::SpAttack),
            Nature::Relaxed => Some(StatType::Defense),
            Nature::Sassy => Some(StatType::SpDefense),
            Nature::Serious => None,
            Nature::Timid => Some(StatType::Speed),
        }
    }

    pub const fn decreases(&self) -> Option<StatType> {
        match self {
            Nature::Adamant => Some(StatType::SpAttack),
            Nature::Bashful => None,
            Nature::Bold => Some(StatType::Attack),
            Nature::Brave => Some(StatType::Speed),
            Nature::Calm => Some(StatType::Attack),
            Nature::Careful => Some(StatType::SpAttack),
            Nature::Docile => None,
            Nature::Gentle => Some(StatType::Defense),
            Nature::Hardy => None,
            Nature::Hasty => Some(StatType::Defense),
            Nature::Impish => Some(StatType::SpAttack),
            Nature::Jolly => Some(StatType::SpAttack),
            Nature::Lax => Some(StatType::SpDefense),
            Nature::Lonely => Some(StatType::Defense),
            Nature::Mild => Some(StatType::Defense),
            Nature::Modest => Some(StatType::Attack),
            Nature::Naive => Some(StatType::SpDefense),
            Nature::Naughty => Some(StatType::SpDefense),
            Nature::Quiet => Some(StatType::Speed),
            Nature::Quirky => None,
            Nature::Rash => Some(StatType::SpDefense),
            Nature::Relaxed => Some(StatType::Speed),
            Nature::Sassy => Some(StatType::Speed),
            Nature::Serious => None,
            Nature::Timid => Some(StatType::Speed),
        }
    }

    pub fn multiplier(&self, stat: &StatType) -> f32 {
        let x = match self.increases().filter(|t| t == stat).is_some() {
            true => 1.1,
            false => 1.0,
        };
        let y = match self.decreases().filter(|t| t == stat).is_some() {
            true => 0.9,
            false => 1.0,
        };
        x * y
    }
}
