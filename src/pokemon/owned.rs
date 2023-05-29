use core::ops::RangeInclusive;

use alloc::{string::String, sync::Arc, vec::Vec};

use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    ailment::LiveAilment,
    item::{Item, ItemId},
    moves::{
        owned::*,
        Move, MoveId, PP,
    },
    pokemon::{
        stat::{BaseStat, StatType, Stats},
        EvolutionType, Experience, Friendship, Gender, Health, Level, Nature, Pokemon, PokemonId,
    },
    Dex,
};

// pub type HP = crate::MaximumNumber<Health>;

#[deprecated(note = "replace with more robust system!")]
const MOVE_MAX: usize = 4;

/// The base struct for a pokemon owned by a player. (But serializable)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPokemonData {
    /// Pokemon Identifier
    pub pokemon: PokemonId,

    /// [Level] of the pokemon (1 - 100)
    pub level: Level,

    /// The [Gender] of this pokemon.
    pub gender: Gender,

    pub nature: Nature,

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
    pub moves: Vec<UserMoveData>,

    #[serde(default)]
    pub item: Option<ItemId>,

    #[serde(default)]
    pub experience: Experience,
}

#[derive(Debug, Clone, Copy)]
pub enum UserPokemonField {
    Pokemon,
    Gender,
    Nature,
    Health,
    Moves,
    Item,
}

#[derive(Debug, Clone, Copy)]
pub enum UserPokemonErrorType {
    Missing,
    CannotInitialize,
}

#[derive(Debug, Clone, Copy)]
pub struct UserPokemonError {
    pub error: UserPokemonErrorType,
    pub reason: UserPokemonField,
}

impl UserPokemonError {

    pub fn missing(reason: UserPokemonField) -> Self {
        Self {
            error: UserPokemonErrorType::Missing,
            reason,
        }
    }

    pub fn init(reason: UserPokemonField) -> Self {
        Self { error: UserPokemonErrorType::CannotInitialize, reason }
    }

}

/// The base struct for a pokemon owned by a player.
#[derive(Debug, Clone)]
pub struct UserPokemon {
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

    pub moves: Vec<UserMove>,

    pub item: Option<Arc<Item>>,

    pub experience: Experience,
}

impl UserPokemon {
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

        while self.moves.len() < MOVE_MAX {
            match moves.next() {
                Some(id) => {
                    if let Some(m) = movedex.try_get(id) {
                        self.moves.push(UserMove::from(m.clone()));
                    }
                }
                None => break,
            }
        }

        moves
    }

    pub fn data(&self) -> UserPokemonData {
        UserPokemonData {
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
            nickname: self.nickname.clone(),
            moves: self.moves.iter().map(UserMove::data).collect(),
            item: self.item.as_ref().map(|item| item.id),
            experience: self.experience,
        }
    }

}


impl UserPokemonData {
    /// Initialize a [SavedPokemon] that already has selfs given to its uninitialized fields
    pub fn init<R: Rng>(
        &self,
        pokedex: &Dex<Pokemon>,
        movedex: &Dex<Move>,
        itemdex: &Dex<Item>,
        mut update: Option<&mut R>,
    ) -> Result<UserPokemon, UserPokemonError> {

        fn generate_or_error<R: Rng, T>(t: Option<T>, update: &mut Option<&mut R>, f: impl FnOnce(&mut R) -> T, field: UserPokemonField) -> Result<T, UserPokemonError> {
            match t {
                Some(t) => Ok(t),
                None => match update {
                    Some(r) => Ok((f)(r)),
                    None => Err(UserPokemonError::init(field)),
                }
            }
        }

        let pokemon = pokedex.try_get(&self.pokemon).ok_or(UserPokemonError::missing(UserPokemonField::Pokemon))?;
        // let gender = generate_or_error(self.gender, update, |random| pokemon.generate_gender(random), UserPokemonField::Gender)?;
        // let nature = self.nature.ok_or_else(())?;
        let hp = self.hp.unwrap_or_else(|| pokemon.stat(&self.ivs, &self.evs, self.level, self.nature, StatType::Health));
        let mut moves = Vec::with_capacity(self.moves.len());

        for m in self.moves.iter() {
            moves.push(m.init(movedex).ok_or(UserPokemonError::init(UserPokemonField::Moves))?);
        }

        let item = self.item.and_then(|ref id| itemdex.try_get(id));

        if update.is_some() && moves.is_empty() {
            let mut m = pokemon
                .moves_at(1..=self.level)
                .rev()
                .flat_map(|id| movedex.try_get(id))
                .collect::<alloc::vec::Vec<_>>();
            m.dedup_by(|a, b| a.id == b.id);
            m.truncate(MOVE_MAX);
            for m in m {
                moves.push(UserMove::from(m.clone()));
            }
        }

        Ok(UserPokemon {
            // data: OwnablePokemonData {
            pokemon: pokemon.clone(),
            level: self.level,
            gender: self.gender,
            nature: self.nature,
            hp,
            ivs: self.ivs,
            evs: self.evs,
            friendship: self.friendship,
            ailment: self.ailment,
            // },
            nickname: self.nickname.clone(),
            moves,
            item: item.cloned(),
            experience: self.experience,
        })
    }

}