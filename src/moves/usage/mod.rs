use serde::{Deserialize, Serialize};

use crate::{
    ailment::{Ailment, AilmentLength},
    pokemon::stat::{Stage, StatType},
};

mod damage;
pub use damage::*;

mod result;
pub use result::*;

pub type Critical = bool;
pub type Percent = u8; // 0 to 100

#[derive(Debug, Clone, Deserialize, Serialize)]
// #[serde(deny_unknown_fields)]
pub enum MoveUseType {
    Damage(DamageKind),
    Ailment(Ailment, AilmentLength, Percent),
    // Ailment(Ailment, f32),
    Drain(DamageKind, i8),
    StatStage(StatType, Stage),
    Flinch,
    Chance(Vec<Self>, Percent),
    User(Vec<Self>),
    Script(String),
    Todo,
}

impl MoveUseType {
    pub(crate) fn usages(&self) -> usize {
        match self {
            MoveUseType::Chance(uses, ..) | MoveUseType::User(uses) => {
                uses.iter().map(Self::usages).sum()
            }
            _ => 1,
        }
    }
}
