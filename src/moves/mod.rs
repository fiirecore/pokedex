use tinystr::{TinyStr16, TinyStr4};
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

use crate::{
    moves::{target::MoveTarget, usage::MoveUseType},
    types::PokemonType,
    id::{Dex, Identifiable, IdentifiableRef},
};

mod category;
pub use category::*;

pub mod instance;

pub mod target;
pub mod usage;

pub mod persistent;

pub type MoveId = TinyStr16;
pub type Power = u8;
pub type Accuracy = u8;
pub type PP = u8;
pub type Priority = i8;
pub type CriticalRate = u8;

pub type FieldMoveId = TinyStr4;

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

impl Identifiable for Move {
    type Id = MoveId;

    fn id(&self) -> &Self::Id {
        &self.id
    }

}

pub struct Movedex;

pub type MoveRef = IdentifiableRef<Movedex>;

static mut MOVEDEX: Option<HashMap<MoveId, Move>> = None;

impl Dex for Movedex {
    type Kind = Move;

    const UNKNOWN: MoveId = crate::UNKNOWN_ID;

    fn dex() -> &'static HashMap<MoveId, Self::Kind> {
        unsafe { MOVEDEX.as_ref().unwrap() }
    }

    fn dex_mut() -> &'static mut Option<HashMap<MoveId, Self::Kind>> {
        unsafe { &mut MOVEDEX }
    }
}

impl core::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
