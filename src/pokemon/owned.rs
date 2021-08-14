use core::fmt::{Display, Formatter, Result as FmtResult};
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    ailment::LiveAilment,
    item::{
        usage::{ItemAction, ItemCondition, ItemUsageKind},
        Item, ItemId, ItemRef, Itemdex,
    },
    moves::{OwnedRefMove, MoveId, MoveRefSet, MoveSet, Movedex, OwnedIdMove, MOVESET_LENGTH, PP},
    pokemon::{
        stat::{BaseStat, StatType, Stats},
        Experience, Friendship, Gender, Health, Level, Pokedex, Pokemon, PokemonId, PokemonRef,
    },
};

pub type OwnedIdPokemon = OwnedPokemon<PokemonId, MoveSet<OwnedIdMove>, ItemId, Option<Health>>;
pub type OwnedRefPokemon<'d, U> = OwnedPokemon<PokemonRef<'d>, MoveRefSet<'d, U>, ItemRef<'d>, Health>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnedPokemon<P, M, I, H> {
    /// Pokemon Identifier
    pub pokemon: P,

    /// Level of the pokemon (1 - 100)
    pub level: Level,

    /// Optional nickname for the pokemon
    #[serde(default)]
    pub nickname: Option<String>,

    #[serde(default)]
    pub gender: Option<Gender>,

    #[serde(default)]
    pub moves: M,

    #[serde(default)]
    pub hp: H,

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

impl OwnedIdPokemon {
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
            hp: Default::default(),
            nickname: Default::default(),
            moves: Default::default(),
            evs: Default::default(),
            item: Default::default(),
            ailment: Default::default(),
            experience: Default::default(),
        }
    }

    pub fn init<'a, U>(
        self,
        random: &mut impl Rng,
        pokedex: &'a Pokedex,
        movedex: &'a Movedex<U>,
        itemdex: &'a Itemdex,
    ) -> Option<OwnedRefPokemon<'a, U>> {
        let pokemon = pokedex.try_get(&self.pokemon)?;
        let hp = self
            .hp
            .unwrap_or_else(|| pokemon.stat(&self.ivs, &self.evs, self.level, StatType::Health));
        let moves = MoveRefSet::new(
            movedex,
            if self.moves.is_empty() {
                pokemon.generate_moves(self.level)
            } else {
                self.moves
            }
            .into_iter()
            .flat_map(|i| i.init(movedex))
            .collect(),
        );
        let item = self.item.map(|ref id| itemdex.try_get(id)).flatten();
        let gender = self.gender.or_else(|| pokemon.generate_gender(random));
        Some(OwnedRefPokemon {
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
            hp,
        })
    }
}

impl<'a, U> OwnedRefPokemon<'a, U> {
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

    pub fn fainted(&self) -> bool {
        self.hp == 0
    }

    pub fn replace_move(&mut self, index: usize, id: &MoveId) {
        if index < MOVESET_LENGTH {
            if let Some(m) = self.moves.movedex.try_get(id) {
                self.moves[index] = OwnedRefMove::new(m);
            }
        }
    }

    pub fn moves_at_level(&self) -> impl Iterator<Item = MoveId> + '_ {
        self.pokemon.moves_at_level(self.level)
    }

    pub fn add_exp(&mut self, experience: Experience) -> impl Iterator<Item = MoveId> + '_ {
        // add exp to pokemon

        self.experience += experience * 5;

        // level the pokemon up if they reach a certain amount of exp (and then subtract the exp by the maximum for the previous level)

        let gr = self.pokemon.training.growth_rate;

        let previous = self.level;

        while self.experience > gr.max_exp(self.level) {
            self.experience -= gr.max_exp(self.level);
            self.level += 1;
        }

        self.on_level_up(previous)
    }

    pub fn exp_from(&self) -> Experience {
        self.pokemon.exp_from(self.level)
    }

    pub fn on_level_up(&mut self, previous: Level) -> impl Iterator<Item = MoveId> + '_ {
        // Get the moves the pokemon learns at the level it just gained.

        let mut moves = self.pokemon.moves_at(previous..self.level).into_iter();

        // Add moves if the player's pokemon does not have a full set of moves.

        while !self.moves.is_full() {
            match moves.next() {
                Some(id) => {
                    if let Some(m) = self.moves.movedex.try_get(&id) {
                        self.moves.push(OwnedRefMove::new(m))
                    }
                }
                None => break,
            }
        }

        moves
    }

    pub fn try_use_item(&mut self, item: &Item) -> bool {
        if !item.usage.conditions.iter().any(|c| match c {
            ItemCondition::Fainted => self.fainted(),
        }) {
            return false;
        }
        match &item.usage.kind {
            ItemUsageKind::Actions(actions) => {
                for action in actions {
                    match action {
                        ItemAction::CurePokemon(status) => {
                            if let Some(effect) = &self.ailment {
                                if let Some(status) = status {
                                    if &effect.ailment == status {
                                        self.ailment = None;
                                    }
                                } else {
                                    self.ailment = None;
                                }
                            }
                        }
                        ItemAction::HealPokemon(hp) => {
                            self.heal_hp(Some(*hp));
                        }
                    }
                }
            }
            ItemUsageKind::Script => log::error!("to-do: item script engines"),
            ItemUsageKind::Pokeball | ItemUsageKind::None => return false,
        }
        true
    }

    pub fn uninit(self) -> OwnedIdPokemon {
        OwnedIdPokemon {
            pokemon: self.pokemon.id,
            level: self.level,
            nickname: self.nickname,
            gender: self.gender,
            moves: self.moves.set.into_iter().map(OwnedRefMove::uninit).collect(),
            hp: Some(self.hp),
            item: self.item.map(|item| item.id),
            ailment: self.ailment,
            ivs: self.ivs,
            evs: self.evs,
            experience: self.experience,
            friendship: self.friendship,
        }
    }

    pub fn use_held_item(&mut self) -> bool {
        match self.item.take() {
            Some(item) => self.try_use_item(&item),
            None => false,
        }
    }
}

impl Display for OwnedIdPokemon {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "ID {}, Lv. {}", self.pokemon, self.level)
    }
}

impl<'a, U> Display for OwnedRefPokemon<'a, U> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Lv. {} {}", self.level, self.pokemon.name)
    }
}
