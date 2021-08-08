use core::fmt::{Display, Formatter, Result as FmtResult};
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    ailment::LiveAilment,
    item::{ItemId, ItemRef, Itemdex},
    moves::{MoveId, MoveInstance, MoveRef, MoveSet, Movedex, MOVESET_LENGTH, PP},
    pokemon::{
        stat::{BaseStat, StatType, Stats},
        Experience, Friendship, Gender, Health, Level, Party, Pokedex, Pokemon, PokemonId,
        PokemonRef,
    },
};

mod exp;
mod item;
mod moves;

pub type UninitPokemon = PokemonInstance<PokemonId, MoveId, ItemId>;
pub type InitPokemon<'a> = PokemonInstance<PokemonRef<'a>, MoveRef<'a>, ItemRef<'a>>;

pub type UninitParty = Party<UninitPokemon>;
pub type InitParty<'a> = Party<InitPokemon<'a>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Test {
    Send(UninitPokemon),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonInstance<P, M, I> {
    /// Pokemon Identifier
    pub pokemon: P,

    /// Level of the pokemon (1 - 100)
    pub level: Level,

    /// Optional nickname for the pokemon
    #[serde(default)]
    pub nickname: Option<String>,

    #[serde(default)]
    pub gender: Option<Gender>,

    #[serde(default = "MoveSet::new")]
    pub moves: MoveSet<MoveInstance<M>>,

    #[serde(default = "UninitPokemon::default_hp_marker")]
    pub hp: Health,

    #[serde(default = "Option::default")]
    pub item: Option<I>,

    #[serde(default)]
    pub ailment: Option<LiveAilment>,

    #[serde(default = "Stats::default_iv")]
    pub ivs: Stats,
    #[serde(default)]
    pub evs: Stats,

    #[serde(default)]
    pub experience: Experience,

    #[serde(default = "Pokemon::default_friendship")]
    pub friendship: Friendship,
}

impl<P, M, I> PokemonInstance<P, M, I> {
    pub fn fainted(&self) -> bool {
        self.hp == 0
    }

    pub fn replace_move(&mut self, index: usize, m: MoveInstance<M>) {
        if index < MOVESET_LENGTH {
            self.moves[index] = m;
        }
    }
}

impl UninitPokemon {
    pub fn generate(
        random: &mut impl Rng,
        pokemon: PokemonId,
        level: Level,
        gender: Option<Gender>,
        ivs: Option<Stats>,
    ) -> Self {
        Self {
            pokemon,
            level,
            gender,
            ivs: ivs.unwrap_or_else(|| Stats::random(random)),
            friendship: Pokemon::default_friendship(),
            hp: Self::default_hp_marker(),
            nickname: Default::default(),
            moves: Default::default(),
            evs: Default::default(),
            item: Default::default(),
            ailment: Default::default(),
            experience: Default::default(),
        }
    }

    pub fn init<'a>(
        self,
        random: &mut impl Rng,
        pokedex: &'a Pokedex,
        movedex: &'a Movedex,
        itemdex: &'a Itemdex,
    ) -> Option<InitPokemon<'a>> {
        let pokemon = pokedex.try_get(&self.pokemon)?;
        let moves = if self.moves.is_empty() {
            pokemon.generate_moves(self.level)
        } else {
            self.moves
        }
        .into_iter()
        .flat_map(|i| i.init(movedex))
        .collect();
        let item = self.item.map(|ref id| itemdex.try_get(id)).flatten();
        let gender = self.gender.or_else(|| pokemon.generate_gender(random));
        Some(InitPokemon {
            pokemon,
            nickname: self.nickname,
            level: self.level,
            gender,
            ivs: self.ivs,
            evs: self.evs,
            experience: self.experience,
            friendship: self.friendship,
            moves,
            ailment: self.ailment,
            item,
            hp: self.hp,
        })
    }

    pub fn default_hp_marker() -> Health {
        Health::MAX
    }
}

impl<'a> InitPokemon<'a> {
    pub fn name<'b: 'a>(&'b self) -> &'b str {
        self.nickname.as_ref().unwrap_or(&self.pokemon.name)
    }

    pub fn hp(&self) -> Health {
        self.hp
    }

    pub fn max_hp(&self) -> Health {
        self.stat(StatType::Health)
    }

    pub fn percent_hp(&self) -> f32 {
        self.hp() as f32 / self.max_hp() as f32
    }

    pub fn stat(&self, stat: StatType) -> BaseStat {
        self.pokemon.stat(&self.ivs, &self.evs, self.level, stat)
    }

    pub fn heal(&mut self, hp: Option<Health>, pp: Option<PP>) {
        self.heal_hp(hp);
        self.heal_pp(pp);
    }

    pub fn heal_hp(&mut self, amount: Option<Health>) {
        let max = self.max_hp();
        self.hp = amount.unwrap_or(max).min(max);
    }

    pub fn heal_pp(&mut self, amount: Option<PP>) {
        self.moves.iter_mut().for_each(|i| i.restore(amount))
    }

    pub fn moves_at_level(&self) -> impl Iterator<Item = MoveId> + '_ {
        self.pokemon.moves_at_level(self.level)
    }
}

impl<'a> From<InitPokemon<'a>> for UninitPokemon {
    fn from(p: InitPokemon<'a>) -> Self {
        Self {
            pokemon: p.pokemon.id,
            level: p.level,
            nickname: p.nickname,
            gender: p.gender,
            moves: p.moves.into_iter().map(Into::into).collect(),
            hp: p.hp,
            item: p.item.map(|item| item.id),
            ailment: p.ailment,
            ivs: p.ivs,
            evs: p.evs,
            experience: p.experience,
            friendship: p.friendship,
        }
    }
}

impl Display for UninitPokemon {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "ID {}, Lv. {}", self.pokemon, self.level)
    }
}

impl<'a> Display for InitPokemon<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Lv. {} {}", self.level, self.pokemon.name)
    }
}
