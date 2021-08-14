use core::fmt::{Display, Formatter, Result as FmtResult};
use serde::{Deserialize, Serialize};
use tinystr::TinyStr16;

use crate::{
    id::{Dex, Identifiable, IdentifiableRef},
    types::PokemonType,
    pokemon::stat::StatType,
};

mod owned;
pub use owned::*;

mod set;
pub use set::*;

pub type MoveId = TinyStr16;
pub type Power = u8;
pub type Accuracy = u8;
pub type PP = u8;
pub type Priority = i8;

pub type MoveRef<'a, U> = IdentifiableRef<'a, Move<U>>;

pub type Movedex<U> = Dex<Move<U>>;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Move<U> {
    pub id: MoveId,

    pub name: String,
    pub category: MoveCategory,
    #[serde(rename = "type")]
    pub pokemon_type: PokemonType,

    pub accuracy: Option<Accuracy>,
    pub pp: PP,
    #[serde(default)]
    pub priority: Priority,

    pub usage: U,

    /// World moves are also known as field moves. This boolean tells if this move is a world move.
    #[serde(default)]
    pub world: bool,
}

impl<U> Identifiable for Move<U> {
    type Id = MoveId;

    const UNKNOWN: Self::Id = crate::id::UNKNOWN_ID;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Deserialize, Serialize)]
pub enum MoveCategory {
    Status,
    Physical,
    Special,
}

impl MoveCategory {
    pub fn stats(&self) -> (StatType, StatType) {
        (self.attack(), self.defense())
    }
    pub fn attack(&self) -> StatType {
        match self {
            MoveCategory::Physical => StatType::Attack,
            MoveCategory::Special => StatType::SpAttack,
            MoveCategory::Status => unreachable!("Cannot get attack stat for status move!"),
        }
    }
    pub fn defense(&self) -> StatType {
        match self {
            MoveCategory::Physical => StatType::Defense,
            MoveCategory::Special => StatType::SpDefense,
            MoveCategory::Status => unreachable!("Cannot get defense stat for status move!"),
        }
    }
}

impl<U> Display for Move<U> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.name, f)
    }
}
