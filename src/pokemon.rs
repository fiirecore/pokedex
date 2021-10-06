//! Types and structs related to Pokemon
//! 

use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::Range,
};
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    moves::{MoveCategory, MoveId, MoveSet, OwnedIdMove},
    types::{Effective, PokemonType},
    Dex, Identifiable, IdRef,
};

mod owned;
pub use owned::*;

mod data;
pub use data::*;

pub mod stat;
use self::stat::{BaseStat, Stat, StatType, Stats};

/// The identifier of a Pokemon.
pub type PokemonId = u16;
/// The level of a pokemon. Usually 1 - 100.
/// Levels determine a Pokemon's power, and higher is better.
pub type Level = u8;
/// How much experience a Pokemon has.
/// Experience is progress between a Pokemon's levels.
pub type Experience = u32;
/// The friendship value of a Pokemon. 0 - 255.
pub type Friendship = u8;
/// The amount of health a pokemon has.
pub type Health = stat::BaseStat;

/// A Pokemon.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pokemon {
    pub id: PokemonId,
    pub name: String,

    pub primary_type: PokemonType,
    pub secondary_type: Option<PokemonType>,

    pub moves: Vec<LearnableMove>,
    pub base: Stats,

    pub species: String,
    pub height: u8,
    pub weight: u16,
    pub training: Training,
    pub breeding: Breeding,
}

/// Common maximum size of a Pokemon party.
pub const PARTY_LENGTH: usize = 6;

/// A type that represents a Pokemon party.
/// A Party is a collection of pokemon a trainer can use.
pub type Party<P> = arrayvec::ArrayVec<[P; PARTY_LENGTH]>;

/// A reference to a Pokemon.
pub type PokemonRef<'d> = IdRef<'d, Pokemon>;

/// Stores Pokemon and can return a reference if given an identifier.
pub type Pokedex = Dex<Pokemon>;

impl Pokemon {
    /// Generate a set of moves given this pokemon and a level.
    pub fn generate_moves(&self, level: Level) -> MoveSet<OwnedIdMove> {
        let mut learnable = self
            .moves
            .iter()
            .filter(|learnable_move| learnable_move.0 <= level)
            .map(|learnable_move| learnable_move.1)
            .rev();

        let mut moves = MoveSet::<OwnedIdMove>::new();

        while !moves.is_full() {
            match learnable.next() {
                Some(m) => {
                    if !moves.iter().any(|i| i.m == m) {
                        moves.push(m.into());
                    }
                }
                None => break,
            }
        }

        moves
    }

    /// Generate a pokemon's gender based on its percent to be a certain gender and a random number generator.
    pub fn generate_gender(&self, random: &mut impl Rng) -> Option<Gender> {
        self.breeding.gender.map(
            |percentage| match random.gen_range(Gender::RANGE) > percentage {
                true => Gender::Male,
                false => Gender::Female,
            },
        )
    }

    /// Check how effective a pokemon type is on this pokemon.
    pub fn effective(&self, user: PokemonType, category: MoveCategory) -> Effective {
        let primary = user.effective(self.primary_type, category);
        if let Some(secondary) = self.secondary_type {
            primary * user.effective(secondary, category)
        } else {
            primary
        }
    }

    /// Get the amount of exp that can be gained from defeating this pokemon at a certain level.
    pub fn exp_from(&self, level: Level) -> Experience {
        ((self.training.base_exp * level as u16) / 7) as Experience
    }

    /// Get the moves of a pokemon at a certain level.
    pub fn moves_at_level(&self, level: Level) -> impl Iterator<Item = MoveId> + '_ {
        self.moves.iter().filter(move |m| m.0 == level).map(|l| l.1)
    }

    /// Get an iterator of the moves a pokemon can get from a range of levels.
    pub fn moves_at(&self, levels: Range<Level>) -> impl Iterator<Item = MoveId> + '_ {
        let levels = Range {
            start: levels.start + 1,
            end: levels.end + 1,
        };

        levels
            .into_iter()
            .flat_map(move |level| self.moves_at_level(level))
    }

    /// Get the value of a base stat from basic stats.
    pub fn stat(&self, ivs: &Stats, evs: &Stats, level: Level, stat: StatType) -> BaseStat {
        match stat {
            StatType::Health => Self::base_hp(self.base.hp, ivs.hp, evs.hp, level),
            stat => Self::base_stat(self.base.get(stat), ivs.get(stat), evs.get(stat), level),
        }
    }

    /// Get the value of a base stat from basic stats, excluding health.
    pub fn base_stat(base: Stat, iv: Stat, ev: Stat, level: Level) -> BaseStat {
        //add item check
        let nature = 1.0;
        (((2.0 * base as f32 + iv as f32 + ev as f32) * level as f32 / 100.0 + 5.0).floor()
            * nature)
            .floor() as BaseStat
    }

    /// Get the base health of a pokemon from basic stats.
    pub fn base_hp(base: Stat, iv: Stat, ev: Stat, level: Level) -> Health {
        ((2.0 * base as f32 + iv as f32 + ev as f32) * level as f32 / 100.0 + level as f32 + 10.0)
            .floor() as Health
    }

    pub const fn default_friendship() -> Friendship {
        70
    }
}

impl Identifiable for Pokemon {
    type Id = PokemonId;

    const UNKNOWN: Self::Id = 0;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl Display for Pokemon {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "#{} {}", self.id, self.name)
    }
}
