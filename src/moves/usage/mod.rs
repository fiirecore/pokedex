use serde::{Deserialize, Serialize};

use crate::{
    pokemon::stat::{BattleStatType, Stage},
    status::{Status, StatusRange},
};

mod damage;
pub use damage::*;

mod result;
pub use result::*;

pub mod script;

pub type Critical = bool;
pub type Percent = u8; // 0 to 100

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum MoveUseType {
    Damage(DamageKind),
    Status(Status, StatusRange, Percent),
    // Ailment(Ailment, f32),
    Drain(DamageKind, i8),
    StatStage(BattleStatType, Stage),
    Flinch,
    Script(String),
    Chance(Vec<Self>, Percent),
    User(Vec<Self>),
    Todo,
}

impl MoveUseType {

    pub(crate) fn usages(&self) -> usize {
        match self {
            MoveUseType::Chance(uses, ..) | MoveUseType::User(uses) => uses.iter().map(Self::usages).sum(),
            _ => 1,
        }
    }

}