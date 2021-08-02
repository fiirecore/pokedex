use arrayvec::ArrayVec;
use core::fmt::{Display, Formatter, Result as FmtResult};
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use tinystr::{TinyStr16, TinyStr4};

use crate::{
    id::{Dex, Identifiable, IdentifiableRef},
    moves::usage::MoveUseType,
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

pub type MoveId = TinyStr16;
pub type Power = u8;
pub type Accuracy = u8;
pub type PP = u8;
pub type Priority = i8;
pub type CriticalRate = u8;

pub type FieldMoveId = TinyStr4;

pub type MoveSet<M> = ArrayVec<[M; 4]>;

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

    pub usage: Vec<MoveUseType>,

    #[serde(default)]
    pub target: MoveTarget,

    #[serde(default)]
    pub contact: bool,

    #[serde(default)]
    pub crit_rate: CriticalRate,

    pub field_id: Option<FieldMoveId>,
}

impl Move {
    pub(crate) fn usages(&self) -> usize {
        self.usage.iter().map(MoveUseType::usages).sum()
    }
}

impl Identifiable for Move {
    type Id = MoveId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

pub struct Movedex;

pub type MoveRef = IdentifiableRef<Movedex>;

#[deprecated(note = "remove static variables")]
static mut MOVEDEX: Option<HashMap<MoveId, Move>> = None;

impl Dex for Movedex {
    type Kind = Move;

    const UNKNOWN: MoveId = crate::id::UNKNOWN_ID;

    fn dex() -> &'static HashMap<MoveId, Self::Kind> {
        unsafe { MOVEDEX.as_ref().unwrap() }
    }

    fn dex_mut() -> &'static mut Option<HashMap<MoveId, Self::Kind>> {
        unsafe { &mut MOVEDEX }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.name)
    }
}
