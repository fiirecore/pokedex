use core::ops::RangeInclusive;

use alloc::{string::String, sync::Arc};

use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    ailment::LiveAilment,
    item::{Item, ItemId},
    moves::{
        owned::{OwnedMove, SavedMove},
        set::MoveSet,
        Move, MoveId, PP,
    },
    pokemon::{
        stat::{BaseStat, StatType, Stats},
        EvolutionType, Experience, Friendship, Gender, Health, Level, Nature, Pokemon, PokemonId,
    },
    Dex,
};

// pub type HP = crate::MaximumNumber<Health>;

/// The base struct for a pokemon owned by a player. (But serializable)
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SavedPokemon {
    /// Pokemon Identifier
    pub pokemon: PokemonId,

    /// [Level] of the pokemon (1 - 100)
    pub level: Level,

    /// The [Gender] of this pokemon.
    #[serde(default)]
    pub gender: Option<Gender>,

    pub nature: Option<Nature>,

    /// The [Health] of this pokemon.
    #[serde(default)]
    pub hp: Option<Health>,

    #[serde(default = "Stats::default_iv")]
    pub ivs: Stats,
    #[serde(default)]
    pub evs: Stats,

    #[serde(default = "Pokemon::default_friendship")]
    pub friendship: Friendship,

    #[serde(default)]
    pub ailment: Option<LiveAilment>,

    /// Optional nickname for the pokemon
    #[serde(default)]
    pub nickname: Option<String>,

    #[serde(default)]
    pub moves: MoveSet<SavedMove>,

    #[serde(default)]
    pub item: Option<ItemId>,

    #[serde(default)]
    pub experience: Experience,
}

/// The base struct for a pokemon owned by a player.
#[derive(Debug, Clone)]
pub struct OwnedPokemon {
    /// Pokemon Identifier
    pub pokemon: Arc<Pokemon>,

    /// [Level] of the pokemon (1 - 100)
    pub level: Level,

    /// The [Gender] of this pokemon.
    // #[serde(default)]
    pub gender: Gender,

    pub nature: Nature,

    /// The [Health] of this pokemon.
    pub hp: Health,

    pub ivs: Stats,
    pub evs: Stats,

    pub friendship: Friendship,

    pub ailment: Option<LiveAilment>,

    /// Optional nickname for the pokemon
    pub nickname: Option<String>,

    pub moves: MoveSet<OwnedMove>,

    pub item: Option<Arc<Item>>,

    pub experience: Experience,
}

impl OwnedPokemon {
    /// Get the current HP of this pokemon.
    pub fn hp(&self) -> Health {
        self.hp
    }

    /// Has the pokemon fainted.
    pub fn fainted(&self) -> bool {
        self.hp == 0
    }

    /// Get the name of this pokemon.
    /// Returns the nickname or the pokemon's name.
    pub fn name(&self) -> &str {
        self.nickname
            .as_deref()
            .unwrap_or(self.pokemon.name.as_str())
    }

    pub fn should_evolve(&self) -> Option<&PokemonId> {
        match &self.pokemon.evolution {
            Some(e) => match &e.0 {
                EvolutionType::Level(level) => match level >= &self.level {
                    true => Some(&e.1),
                    false => None,
                },
                // To - do
                _ => None,
            },
            None => None,
        }
    }

    /// Get the [Experience] from this pokemon at its current [Level].
    pub fn exp_from(&self) -> Experience {
        self.pokemon.exp_from(self.level)
    }

    /// Get this pokemon's moves at its current [Level].
    pub fn moves_at_level(&self) -> impl DoubleEndedIterator<Item = &MoveId> + '_ {
        self.pokemon.moves_at_level(self.level)
    }

    /// Get the maximum [Health] of this pokemon.
    pub fn max_hp(&self) -> Health {
        self.stat(StatType::Health)
    }

    /// Get the current [Health] of this pokemon as a percentage.
    pub fn percent_hp(&self) -> f32 {
        self.hp() as f32 / self.max_hp() as f32
    }

    /// Get a [BaseStat] for this pokemon.
    pub fn stat(&self, stat: StatType) -> BaseStat {
        self.pokemon
            .stat(&self.ivs, &self.evs, self.level, self.nature, stat)
    }

    /// Heal this pokemon with an optional amount of [Health].
    pub fn heal_hp(&mut self, amount: Option<Health>) {
        let max = self.max_hp();
        self.hp = amount.unwrap_or(max).min(max);
    }

    /// Heal this pokemon with an optional amount of [Health] and restore all its move's [PP] by an optional amount.
    pub fn heal(&mut self, hp: Option<Health>, pp: Option<PP>) {
        self.heal_hp(hp);
        self.moves.iter_mut().for_each(|o| o.restore(pp));
    }

    /// Add [Experience] to this pokemon, and also handle level ups.
    pub fn add_exp(
        &mut self,
        experience: Experience,
    ) -> RangeInclusive<Level> {
        // add exp to pokemon

        self.experience += experience * 5;

        // level the pokemon up if they reach a certain amount of exp (and then subtract the exp by the maximum for the previous level)

        let previous = self.level;

        let gr = &self.pokemon.training.growth;

        while self.experience > gr.max_exp(self.level) {
            self.experience -= gr.max_exp(self.level);
            self.level += 1;
        }

        previous..=self.level
    }

    /// Handle leveling up.
    pub fn fill_moves<'m>(
        &mut self,
        mut moves: impl DoubleEndedIterator<Item = &'m MoveId> + 'm,
        movedex: &Dex<Move>,
    ) -> impl DoubleEndedIterator<Item = &'m MoveId> + 'm {

        // Add moves if the player's pokemon does not have a full set of moves.

        while !self.moves.is_full() {
            match moves.next() {
                Some(id) => {
                    if let Some(m) = movedex.try_get(id) {
                        self.moves.add(None, m.clone());
                    }
                }
                None => break,
            }
        }

        moves
    }

    pub fn uninit(self) -> SavedPokemon {
        SavedPokemon {
            // data: OwnablePokemonData {
            pokemon: self.pokemon.id,
            level: self.level,
            gender: self.gender.into(),
            nature: self.nature.into(),
            hp: self.hp.into(),
            ivs: self.ivs,
            evs: self.evs,
            friendship: self.friendship,
            ailment: self.ailment,
            // },
            nickname: self.nickname,
            moves: self.moves.uninit(),
            item: self.item.map(|item| item.id),
            experience: self.experience,
        }
    }
}

impl SavedPokemon {
    /// Initialize a [SavedPokemon] that already has values given to its uninitialized fields
    pub fn try_init(
        self,
        pokedex: &Dex<Pokemon>,
        movedex: &Dex<Move>,
        itemdex: &Dex<Item>,
    ) -> Option<OwnedPokemon> {
        let pokemon = pokedex.try_get(&self.pokemon)?;
        let gender = self.gender?;
        let nature = self.nature?;
        let hp = self.hp?;
        let moves = self.moves.init(movedex)?;
        let item = self.item.and_then(|ref id| itemdex.try_get(id));
        Some(OwnedPokemon {
            // data: OwnablePokemonData {
            pokemon: pokemon.clone(),
            level: self.level,
            gender,
            nature,
            hp,
            ivs: self.ivs,
            evs: self.evs,
            friendship: self.friendship,
            ailment: self.ailment,
            // },
            nickname: self.nickname,
            moves,
            item: item.cloned(),
            experience: self.experience,
        })
    }

    /// Initialize this owned pokemon struct into an [OwnedPokemon] so it can perform more functions.
    pub fn init<R: Rng>(
        self,
        random: &mut R,
        pokedex: &Dex<Pokemon>,
        movedex: &Dex<Move>,
        itemdex: &Dex<Item>,
    ) -> Option<OwnedPokemon> {
        let pokemon = pokedex.try_get(&self.pokemon)?;
        let gender = self
            .gender
            .unwrap_or_else(|| pokemon.generate_gender(random));
        let nature = self
            .nature
            .unwrap_or_else(|| Pokemon::generate_nature(random));
        let hp = self.hp.unwrap_or_else(|| {
            pokemon.stat(&self.ivs, &self.evs, self.level, nature, StatType::Health)
        });

        let mut moves = self.moves.init(movedex)?;

        // Replace with iterator dedup soon

        if moves.is_empty() {
            let mut m = pokemon
                .moves_at(1..=self.level)
                .rev()
                .flat_map(|id| movedex.try_get(id))
                .collect::<alloc::vec::Vec<_>>();
            m.dedup_by(|a, b| a.id == b.id);
            m.truncate(4);
            for m in m {
                moves.add(None, m.clone());
            }
        }

        let item = self.item.and_then(|ref id| itemdex.try_get(id));

        Some(OwnedPokemon {
            // data: OwnablePokemonData {
            pokemon: pokemon.clone(),
            level: self.level,
            gender,
            nature,
            hp,
            ivs: self.ivs,
            evs: self.evs,
            friendship: self.friendship,
            ailment: self.ailment,
            // },
            nickname: self.nickname,
            moves,
            item: item.cloned(),
            experience: self.experience,
        })
    }
}

// #[derive(Clone, Debug, PartialEq, Eq)]
// pub struct SavedPokemonGenerator {
//     pub gender: Option<Gender>,
//     pub nature: Option<Nature>,
//     pub hp: Option<Health>,
//     pub ivs: Stats,
//     pub evs: Stats,
//     pub friendship: Friendship,
//     pub ailment: Option<LiveAilment>,
//     pub nickname: Option<String>,
//     // pub moves,
//     // pub item:
//     pub experience: Experience,
// }

// impl Default for SavedPokemonGenerator {
//     fn default() -> Self {
//         Self {
//             gender: Default::default(),
//             nature: Default::default(),
//             hp: Default::default(),
//             friendship: Pokemon::default_friendship(),
//             ivs: Default::default(),
//             evs: Default::default(),
//             ailment: Default::default(),
//             nickname: Default::default(),
//             experience: Default::default(),
//         }
//     }
// }

// impl PokemonView<SavedMove> for SavedPokemon {
//     // type Generator = SavedPokemonGenerator;

//     fn fainted(&self) -> bool {
//         SavedPokemon::fainted(self)
//     }

//     fn id(&self) -> PokemonId {
//         self.pokemon
//     }

//     fn moves(&self) -> core::slice::Iter<SavedMove> {
//         self.moves.iter()
//     }

//     // fn generate(id: PokemonId, level: Level, other: Self::Generator) -> Self
//     // where
//     //     Self: Sized,
//     // {
//     //     Self {
//     //         pokemon: id,
//     //         level,
//     //         gender: other.gender,
//     //         nature: other.nature,
//     //         hp: other.hp,
//     //         ivs: other.ivs,
//     //         evs: other.evs,
//     //         friendship: other.friendship,
//     //         ailment: other.ailment,
//     //         nickname: other.nickname,
//     //         moves: Default::default(),
//     //         item: None,
//     //         experience: other.experience,
//     //     }
//     // }
// }

// #[derive(Clone, Debug, PartialEq, Eq)]
// pub struct OwnedPokemonGenerator<'d, P, M, I> {
//     pub gender: Gender,
//     pub nature: Nature,
//     pub hp: Option<Health>,
//     pub ivs: Stats,
//     pub evs: Stats,
//     pub friendship: Friendship,
//     pub ailment: Option<LiveAilment>,
//     pub nickname: Option<String>,
//     // pub moves,
//     // pub item:
//     pub experience: Experience,
// }

// impl<P: Deref<Target = Pokemon>, M: Deref<Target = Move>, I> PokemonView<OwnedMove<M>>
//     for OwnedPokemon<P, M, I>
// {
//     fn fainted(&self) -> bool {
//         OwnedPokemon::<P, M, I>::fainted(self)
//     }

//     fn id(&self) -> PokemonId {
//         self.pokemon.id
//     }

//     fn moves(&self) -> core::slice::Iter<OwnedMove<M>> {
//         self.moves.iter()
//     }
// }

// impl<P, M, I, H> Deref for OwnablePokemon<P, M, I, H> {
//     type Target = OwnablePokemonData<P, H>;

//     fn deref(&self) -> &Self::Target {
//         &self
//     }
// }

// impl<P, M, I, H> DerefMut for OwnablePokemon<P, M, I, H> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self
//     }
// }
