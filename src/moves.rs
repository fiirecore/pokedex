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

mod target;
pub use target::*;

/// A Move's identifier
pub type MoveId = TinyStr16;
pub type Power = u8;
pub type Accuracy = u8;
pub type PP = u8;
pub type Priority = i8;

pub type MoveRef<'a> = IdentifiableRef<'a, Move>;

pub type Movedex = Dex<Move>;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Move {

    /// A Move's identifier
    pub id: MoveId,

    /// The name of the move.
    pub name: String,

    /// The category of a move.
    pub category: MoveCategory,
    #[serde(rename = "type")]
    pub pokemon_type: PokemonType,

    /// The chance of a move to land.
    /// Holds a value of 1 - 100.
    /// If it is [None], the move will always land.
    pub accuracy: Option<Accuracy>,
    pub power: Option<Power>,
    pub pp: PP,
    #[serde(default)]
    pub priority: Priority,

    /// The target of the move.
    #[serde(default)]
    pub target: target::MoveTarget,

    /// World moves are also known as field moves. This boolean tells if this move is a world move.
    #[serde(default)]
    pub world: bool,
}

impl Identifiable for Move {
    type Id = MoveId;

    const UNKNOWN: Self::Id = crate::id::UNKNOWN_ID;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.name, f)
    }
}

// use MoveCategory::{Physical, Special, Status};
// use crate::pokemon::stat::StatType::{Attack, Defense, SpAttack, SpDefense};

/// The category of a move.
// /// [Physical] and [Special] are usually for moves that deal damage.
// /// [Physical] deals physical damage ([Attack]) against a target pokemon's [Defense].
// /// [Special] deals special damage ([SpAttack]) against a target pokemon's [SpDefense].
// /// [MoveCategory::Status] moves usually afflict an ailment on a target pokemon or benefit the user pokemon.
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