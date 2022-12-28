//! Types and structs related to moves
//!
//! This module does not contain ways to execute moves, as the [battle](https://crates.io/crates/firecore-battle) crate does this.
//!

use alloc::string::String;
use serde::{Deserialize, Serialize};

use crate::{Identifiable, UNKNOWN_ID};

pub mod owned;
pub mod set;

/// How many times a [Move] can be used before needing to be restored.
pub type PP = u8;

type IdInner = tinystr::TinyAsciiStr<16>;

/// An identifier for a [Move].
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(transparent)]
pub struct MoveId(pub IdInner);

/// Moves that Pokemon use in battle.
/// These can also have other uses too, such as triggering events in a world.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Move {
    pub id: MoveId,

    pub name: String,

    pub pp: PP,
}

// impl Identifier<Move> for MoveId {
//     fn as_id(&self) -> &<Move as Identifiable>::Id {
//         self
//     }
// }

impl Identifiable for Move {
    type Id = MoveId;

    const UNKNOWN: Self::Id = MoveId(UNKNOWN_ID);

    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl Default for MoveId {
    fn default() -> Self {
        Move::UNKNOWN
    }
}

impl From<IdInner> for MoveId {
    fn from(inner: IdInner) -> Self {
        Self(inner)
    }
}

impl core::str::FromStr for MoveId {
    type Err = tinystr::TinyStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(Self)
    }
}
