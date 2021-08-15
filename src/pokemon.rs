use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::Range,
};
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    id::{Dex, Identifiable, IdentifiableRef},
    moves::{MoveCategory, MoveId, MoveSet, OwnedIdMove},
    types::{Effective, PokemonType},
};

mod owned;
pub use owned::*;

mod data;
pub use data::*;

pub mod stat;
use self::stat::{BaseStat, Stat, StatType, Stats};

pub type PokemonId = <Pokemon as Identifiable>::Id;
pub type Level = u8;
pub type Experience = u32;
pub type Friendship = u8;
pub type Health = stat::BaseStat;

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

pub const PARTY_LENGTH: usize = 6;

pub type Party<P> = arrayvec::ArrayVec<[P; PARTY_LENGTH]>;

pub type PokemonRef<'a> = IdentifiableRef<'a, Pokemon>;

pub type Pokedex = Dex<Pokemon>;

impl Pokemon {
    pub fn generate_moves(&self, level: Level) -> MoveSet<OwnedIdMove> {
        let mut learnable = self
            .moves
            .iter()
            .filter(|learnable_move| learnable_move.level <= level)
            .map(|learnable_move| learnable_move.id)
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

    pub fn generate_gender(&self, random: &mut impl Rng) -> Option<Gender> {
        self.breeding.gender.map(
            |percentage| match random.gen_range(Gender::RANGE) > percentage {
                true => Gender::Male,
                false => Gender::Female,
            },
        )
    }

    pub fn effective(&self, user: PokemonType, category: MoveCategory) -> Effective {
        let primary = user.effective(self.primary_type, category);
        if let Some(secondary) = self.secondary_type {
            primary * user.effective(secondary, category)
        } else {
            primary
        }
    }

    pub fn exp_from(&self, level: Level) -> Experience {
        ((self.training.base_exp * level as u16) / 7) as Experience
    }

    pub fn moves_at_level(&self, level: Level) -> impl Iterator<Item = MoveId> + '_ {
        self.moves
            .iter()
            .filter(move |m| m.level == level)
            .map(|l| l.id)
    }

    pub fn moves_at(&self, levels: Range<Level>) -> impl Iterator<Item = MoveId> + '_ {
        let levels = Range {
            start: levels.start + 1,
            end: levels.end + 1,
        };

        levels
            .into_iter()
            .flat_map(move |level| self.moves_at_level(level))
    }

    pub fn stat(&self, ivs: &Stats, evs: &Stats, level: Level, stat: StatType) -> BaseStat {
        match stat {
            StatType::Health => Self::base_hp(self.base.hp, ivs.hp, evs.hp, level),
            stat => Self::base_stat(self.base.get(stat), ivs.get(stat), evs.get(stat), level),
        }
    }

    pub fn base_stat(base: Stat, iv: Stat, ev: Stat, level: Level) -> BaseStat {
        //add item check
        let nature = 1.0;
        (((2.0 * base as f32 + iv as f32 + ev as f32) * level as f32 / 100.0 + 5.0).floor()
            * nature)
            .floor() as BaseStat
    }

    pub fn base_hp(base: Stat, iv: Stat, ev: Stat, level: Level) -> BaseStat {
        ((2.0 * base as f32 + iv as f32 + ev as f32) * level as f32 / 100.0 + level as f32 + 10.0)
            .floor() as BaseStat
    }

    pub const fn default_friendship() -> Friendship {
        70
    }
}

impl Identifiable for Pokemon {
    type Id = u16;

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
