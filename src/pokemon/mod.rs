use core::{ops::Range, fmt::{Debug, Display, Formatter, Result as FmtResult}};

use crate::{
    id::{Dex, Identifiable, IdentifiableRef},
    moves::{
        instance::{MoveInstance, MoveInstanceSet},
        MoveRef, Movedex,
    },
    pokemon::{
        data::{Breeding, Gender, LearnableMove, PokedexData, Training},
        stat::Stats,
    },
    types::PokemonType,
};
use hashbrown::HashMap;
use rand::Rng;
use serde::{Deserialize, Serialize};

mod instance;
mod party;

pub use instance::*;
pub use party::*;

pub mod data;
pub mod stat;

pub type PokemonId = u16;
pub type Level = u8;
pub type Experience = u32;
pub type Friendship = u8;
pub type Health = stat::BaseStat;

#[derive(Serialize, Deserialize)]
pub struct Pokemon {
    pub id: PokemonId,
    pub name: String,

    pub primary_type: PokemonType,
    pub secondary_type: Option<PokemonType>,

    pub base: Stats,

    pub data: PokedexData,

    pub training: Training,
    pub breeding: Breeding,

    pub moves: Vec<LearnableMove>,
}

impl Pokemon {
    pub fn generate_moves(&self, level: Level) -> MoveInstanceSet {
        let mut moves2 = self
            .moves
            .iter()
            .filter(|learnable_move| learnable_move.level <= level)
            .map(|learnable_move| learnable_move.id)
            .rev()
            .flat_map(|id| Movedex::try_get(&id))
            .map(MoveInstance::new)
            .collect::<Vec<MoveInstance>>();
        moves2.dedup();
        moves2.truncate(4);
        let mut moves = MoveInstanceSet::new();
        moves.extend(moves2);
        moves
    }

    pub fn generate_gender(&self, random: &mut impl Rng) -> Gender {
        match self.breeding.gender {
            Some(percentage) => match random.gen_range(Gender::RANGE) > percentage {
                true => Gender::Male,
                false => Gender::Female,
            },
            None => Gender::None,
        }
    }

    pub fn exp_from(&self, level: Level) -> Experience {
        ((self.training.base_exp * level as u16) / 7) as Experience
    }

    pub fn moves_at_level(&self, level: Level) -> Vec<MoveRef> {
        self.moves
            .iter()
            .filter(|m| m.level == level)
            .flat_map(|m| Movedex::try_get(&m.id))
            .collect()
    }

    pub fn moves_at(&self, levels: Range<Level>) -> Vec<MoveRef> {
        let levels = Range {
            start: levels.start + 1,
            end: levels.end + 1,
        };

        let mut moves = Vec::new();

        levels.for_each(|level| moves.extend(self.moves_at_level(level)));

        moves
    }
}

impl Identifiable for Pokemon {
    type Id = PokemonId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

pub struct Pokedex;

pub type PokemonRef = IdentifiableRef<Pokedex>;

#[deprecated(note = "remove static variables")]
static mut POKEDEX: Option<HashMap<PokemonId, Pokemon>> = None;

impl Dex for Pokedex {
    type Kind = Pokemon;

    const UNKNOWN: PokemonId = 0;

    fn dex() -> &'static HashMap<PokemonId, Self::Kind> {
        unsafe { POKEDEX.as_ref().unwrap() }
    }

    fn dex_mut() -> &'static mut Option<HashMap<PokemonId, Self::Kind>> {
        unsafe { &mut POKEDEX }
    }
}

pub fn default_iv() -> Stats {
    Stats::uniform(15)
}

pub const fn default_friendship() -> Friendship {
    70
}

impl Debug for Pokemon {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self, f)
    }
}

impl Display for Pokemon {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{} ({})", self.name, self.id)
    }
}
