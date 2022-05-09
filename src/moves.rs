//! Types and structs related to moves
//!
//! This module does not contain ways to execute moves, as the [battle](https://crates.io/crates/firecore-battle) crate does this.
//!

use alloc::string::String;
use serde::{Deserialize, Serialize};
use tinystr::TinyStr16;

use crate::{pokemon::stat::StatType, types::PokemonType, Identifiable, Nameable, UNKNOWN_ID};

pub mod owned;
pub mod set;

/// An identifier for a [Move].
pub type MoveId = TinyStr16;
/// How powerful a [Move] is, in points. Some moves do not use power levels.
pub type Power = u8;
/// How accurate a [Move] is, in values 0 - 100.
pub type Accuracy = u8;
/// How many times a [Move] can be used before needing to be restored.
pub type PP = u8;
/// This determines whether the [Move] goes before another.
/// The higher the value, the higher the priority.
pub type Priority = i8;
/// This helps determine if a [Move] should be a critical hit.
/// The higher the value, the higher the chance of a critical hit.
/// This maxes out at 4.
pub type CriticalRate = u8;

/// Moves that Pokemon use in battle.
/// These can also have other uses too, such as triggering events in a world.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Move {
    pub id: MoveId,

    pub name: String,

    pub category: MoveCategory,

    /// The move's type.
    #[serde(rename = "type")]
    pub pokemon_type: PokemonType,
    /// If this is [None], the move will always land.
    pub accuracy: Option<Accuracy>,
    pub power: Option<Power>,
    pub pp: PP,
    #[serde(default)]
    pub priority: Priority,

    #[serde(default)]
    pub target: MoveTarget,

    /// If the move makes contact with the target.
    #[serde(default)]
    pub contact: bool,

    /// Increments the chance of whether a move should critical hit or not.
    #[serde(default)]
    pub crit_rate: CriticalRate,
}

impl Identifiable for Move {
    type Id = MoveId;

    const UNKNOWN: Self::Id = UNKNOWN_ID;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl Nameable for Move {
    fn name(&self) -> &str {
        &self.name
    }
}

// use MoveCategory::{Physical, Special, Status};
// use crate::pokemon::stat::StatType::{Attack, Defense, SpAttack, SpDefense};

/// The category of a move.
// /// [MoveCategory::Physical] and [MoveCategory::Special] are usually for moves that deal damage.
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
    /// Get a tuple of the attack and defense types of this category.
    pub fn stats(&self) -> (StatType, StatType) {
        (self.attack(), self.defense())
    }
    /// Get the attack type of this category.
    pub fn attack(&self) -> StatType {
        match self {
            MoveCategory::Physical => StatType::Attack,
            MoveCategory::Special => StatType::SpAttack,
            MoveCategory::Status => unreachable!("Cannot get attack stat for status move!"),
        }
    }
    /// Get the defense type of this category.
    pub fn defense(&self) -> StatType {
        match self {
            MoveCategory::Physical => StatType::Defense,
            MoveCategory::Special => StatType::SpDefense,
            MoveCategory::Status => unreachable!("Cannot get defense stat for status move!"),
        }
    }
}

/// The target of a [Move].
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum MoveTarget {
    Any,
    Ally,
    Allies,
    UserOrAlly,
    UserAndAllies,
    // UserOrAllies,
    User,
    Opponent,
    AllOpponents,
    RandomOpponent,
    AllOtherPokemon,
    AllPokemon,
    None,
}

impl Default for MoveTarget {
    fn default() -> Self {
        Self::None
    }
}
