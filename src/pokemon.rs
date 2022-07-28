//! Types and structs related to Pokemon
//!

use alloc::{string::String, vec::Vec};
use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::RangeBounds,
};

use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    moves::{MoveCategory, MoveId},
    types::{Effective, PokemonType, Types},
    Identifiable,
};

pub mod owned;

pub mod party;

pub mod data;
use self::data::*;

pub mod stat;
use self::stat::{BaseStat, Stat, StatType, Stats};

mod nature;
pub use self::nature::*;

/// The identifier of a Pokemon.
pub type PokemonId = u16;
/// The form of a Pokemon.
pub type PokemonFormId = tinystr::TinyStr8;
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

/// A form of a Pokemon.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Pokemon {
    pub id: <Self as Identifiable>::Id,
    pub name: String,

    pub types: Types,

    pub moves: Vec<LearnableMove>,
    pub base: Stats,

    pub species: String,

    #[serde(default)]
    pub evolution: Option<Evolution>,

    pub height: u8,
    pub weight: u16,

    pub training: Training,
    pub breeding: Breeding,
}

impl Pokemon {
    /// Generate a pokemon's [Gender] based on its percent to be a certain gender and a random number generator.
    pub fn generate_gender(&self, random: &mut impl Rng) -> Gender {
        self.breeding
            .gender
            .map(
                |percentage| match random.gen_range(Gender::RANGE) > percentage {
                    true => Gender::Male,
                    false => Gender::Female,
                },
            )
            .unwrap_or(Gender::None)
    }

    pub fn generate_nature(random: &mut impl Rng) -> Nature {
        match random.gen_range(0..Nature::COUNT) {
            0 => Nature::Adamant,
            1 => Nature::Bashful,
            2 => Nature::Bold,
            3 => Nature::Brave,
            4 => Nature::Calm,
            5 => Nature::Careful,
            6 => Nature::Docile,
            7 => Nature::Gentle,
            8 => Nature::Hardy,
            9 => Nature::Hasty,
            10 => Nature::Impish,
            11 => Nature::Jolly,
            12 => Nature::Lax,
            13 => Nature::Lonely,
            14 => Nature::Mild,
            15 => Nature::Modest,
            16 => Nature::Naive,
            17 => Nature::Naughty,
            18 => Nature::Quiet,
            19 => Nature::Quirky,
            20 => Nature::Rash,
            21 => Nature::Relaxed,
            22 => Nature::Sassy,
            23 => Nature::Serious,
            24 => Nature::Timid,
            _ => unreachable!(),
        }
    }

    /// Test how [Effective] a [PokemonType] is on this pokemon, in a specified [MoveCategory].
    pub fn effective(&self, user: PokemonType, category: MoveCategory) -> Effective {
        let primary = user.effective(self.types.primary, category);
        if let Some(secondary) = self.types.secondary {
            primary * user.effective(secondary, category)
        } else {
            primary
        }
    }

    /// Get the amount of [Experience] that can be gained from defeating this pokemon at a certain [Level].
    pub const fn exp_from(&self, level: Level) -> Experience {
        ((self.training.base_exp * level as u16) / 7) as Experience
    }

    /// Get the moves of a pokemon at a certain [Level].
    pub fn moves_at_level(&self, level: Level) -> impl DoubleEndedIterator<Item = &MoveId> + '_ {
        self.moves_at(level..=level)
    }

    /// Get an iterator of the moves a pokemon can get from a range of levels.
    pub fn moves_at<'s, R: RangeBounds<Level> + 's>(
        &'s self,
        levels: R,
    ) -> impl DoubleEndedIterator<Item = &'s MoveId> + 's {
        self.moves
            .iter()
            .filter(move |m| levels.contains(&m.0))
            .map(|m| &m.1)
    }

    /// Get the value of a [BaseStat] from basic stats.
    pub fn stat(
        &self,
        ivs: &Stats,
        evs: &Stats,
        level: Level,
        nature: Nature,
        stat: StatType,
    ) -> BaseStat {
        match stat {
            StatType::Health => Self::base_hp(
                self.base[StatType::Health],
                ivs[StatType::Health],
                evs[StatType::Health],
                level,
            ),
            stat => Self::base_stat(
                self.base[stat],
                ivs[stat],
                evs[stat],
                level,
                nature.multiplier(&stat) as _,
            ),
        }
    }

    /// Get the value of a [BaseStat] from basic stats, excluding health.
    pub fn base_stat(base: Stat, iv: Stat, ev: Stat, level: Level, multiplier: f64) -> BaseStat {
        let (.., mut base) = Self::base(base, iv, ev, level);
        base += 5.0;
        base *= multiplier;
        base as _
    }

    /// Get the base [Health] of a pokemon from basic stats.
    pub fn base_hp(base: Stat, iv: Stat, ev: Stat, level: Level) -> Health {
        let (level, mut base) = Self::base(base, iv, ev, level);
        base += level;
        base += 10.0;
        base as _
    }

    /// returns level and partially completed base stat
    fn base(base: Stat, iv: Stat, ev: Stat, level: Level) -> (f64, f64) {
        let level = level as f64;
        let mut base = 2.0 * base as f64 + iv as f64;
        base += ev as f64 / 4.0;
        base *= level;
        base /= 100.0;
        (level, base)
    }

    /// The default [Friendship] of a pokemon.
    pub const fn default_friendship() -> Friendship {
        70
    }
}

// impl Identifier<Pokemon> for PokemonId {
//     fn as_id(&self) -> &<Pokemon as Identifiable>::Id {
//         self
//     }
// }

impl Identifiable for Pokemon {
    type Id = PokemonId;

    const UNKNOWN: Self::Id = 0;

    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl Display for Pokemon {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "#{} {}", self.id, self.name)
    }
}

use enum_map::Enum;

#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Enum, Deserialize, Serialize,
)]
pub enum PokemonTexture {
    Front,
    Back,
    Icon,
}

#[cfg(test)]
mod tests {

    use crate::{
        item::Item,
        moves::{Move, MoveCategory, MoveTarget, Power, PP},
        pokemon::{
            data::{Breeding, LearnableMove, Training},
            owned::SavedPokemon,
            stat::{StatSet, StatType},
            Nature, Pokemon,
        },
        types::{PokemonType, Types},
        Dex,
    };

    #[test]
    fn stat() {
        let attack = Pokemon::base_stat(
            105,
            15,
            50,
            50,
            Nature::Adamant.multiplier(&StatType::Attack) as _,
        );
        println!("{}", attack);
        assert!(attack == 136);
    }

    #[test]
    fn dex() {
        let mut pokedex = Dex::<Pokemon>::default();

        let test = "test".parse().unwrap();

        let v = Pokemon {
            id: 0,
            name: "Test".to_owned(),
            types: Types {
                primary: PokemonType::Bug,
                secondary: Some(PokemonType::Dragon),
            },
            moves: vec![LearnableMove(1, test)],
            base: StatSet::uniform(60),
            species: "Test Species".to_owned(),
            evolution: None,
            height: 6_5,
            weight: 100,
            training: Training {
                base_exp: 200,
                growth: Default::default(),
            },
            breeding: Breeding { gender: None },
        };

        pokedex.insert(v);

        let mut movedex = Dex::<Move>::default();

        let v = Move {
            id: test,
            name: "Test Move".to_owned(),
            category: MoveCategory::Physical,
            pokemon_type: PokemonType::Bug,
            accuracy: None,
            power: Some(Power::MAX),
            pp: PP::MAX,
            priority: 0,
            target: MoveTarget::Opponent,
            contact: false,
            crit_rate: 1,
        };

        movedex.insert(v);

        let itemdex = Dex::<Item>::default();

        let pokemon = SavedPokemon {
            pokemon: 0,
            level: 30,
            ..Default::default()
        };

        let mut rng = rand::rngs::mock::StepRng::new(12, 24);

        let pokemon = pokemon
            .init(&mut rng, &pokedex, &movedex, &itemdex)
            .unwrap();

        assert!(!pokemon.moves.is_empty())
    }
}
