use arrayvec::ArrayVec;
use core::fmt::{Display, Formatter, Result as FmtResult};
use serde::{Deserialize, Serialize};
use tinystr::{TinyStr16, TinyStr4};

use crate::{
    id::{Dex, Identifiable, IdentifiableRef},
    moves::usage::MoveUsage,
    types::PokemonType,
};

mod category;
pub use category::*;

mod target;
pub use target::*;

mod instance;
pub use instance::*;

pub mod persistent;
pub mod usage;

pub mod script;

pub type MoveId = <Move as Identifiable>::Id;
pub type Power = u8;
pub type Accuracy = u8;
pub type PP = u8;
pub type Priority = i8;
pub type CriticalRate = u8;

pub type FieldMoveId = TinyStr4;

pub const MOVESET_LENGTH: usize = 4;

pub type MoveSet<M> = ArrayVec<[M; MOVESET_LENGTH]>;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Move {
    pub id: MoveId,

    pub name: String,
    pub category: MoveCategory,
    #[serde(rename = "type")]
    pub pokemon_type: PokemonType,

    pub accuracy: Option<Accuracy>,
    pub pp: PP,
    #[serde(default)]
    pub priority: Priority,

    pub usage: MoveUsage,

    #[serde(default)]
    pub target: MoveTarget,

    #[serde(default)]
    pub contact: bool,

    #[serde(default)]
    pub crit_rate: CriticalRate,

    pub field_id: Option<FieldMoveId>,
}

impl Identifiable for Move {
    type Id = TinyStr16;

    const UNKNOWN: Self::Id = crate::id::UNKNOWN_ID;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

pub type MoveRef<'a> = IdentifiableRef<'a, Move>;

pub type Movedex = Dex<Move>;

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.name)
    }
}
