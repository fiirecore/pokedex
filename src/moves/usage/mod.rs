use serde::{Deserialize, Serialize};

use crate::{
    ailment::{Ailment, AilmentLength},
    pokemon::stat::{Stage, StatType},
};

mod damage;
pub use damage::*;

pub mod result;
pub use result::*;

pub type Critical = bool;
pub type Percent = u8; // 0 to 100

pub type MoveUsage = Targets<MoveUsageKind>;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Targets<T> {
    pub user: T,
    pub target: T,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum MoveUsageKind {
    /// Load a vector of actions
    Actions(Vec<MoveAction>),
    /// Use a script defined in the instance of the object that uses this
    Script,
    /// Placeholder to show that object does not have a defined use yet.
    Todo,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum MoveAction {
    Damage(DamageKind),
    Ailment(Ailment, AilmentLength, Percent),
    Drain(DamageKind, i8),
    Stat(StatType, Stage),
    Accuracy(Stage),
    Evasion(Stage),
    Flinch,
    Chance(Vec<Self>, Percent),
}

impl MoveUsageKind {
    pub fn len(&self) -> usize {
        match self {
            Self::Actions(actions) => actions.iter().map(MoveAction::len).sum(),
            Self::Script => 0,
            Self::Todo => 1,
        }
    }

}

impl MoveAction {
    pub fn len(&self) -> usize {
        match self {
            Self::Chance(uses, ..) => uses.iter().map(Self::len).sum(),
            _ => 1,
        }
    }
}
