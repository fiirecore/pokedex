use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    ailment::LiveAilment,
    item::{
        usage::{ItemAction, ItemCondition, ItemUsageKind},
        Item, ItemId,
    },
    moves::{
        set::{OwnedMoveSet, SavedMoveSet},
        Move, MoveId, PP,
    },
    pokemon::{
        stat::{BaseStat, StatType, Stats},
        Experience, Friendship, Gender, Health, Level, Pokemon, PokemonId,
    },
    Dex, Identifiable, Initializable, Uninitializable,
};

// pub type HP = crate::MaximumNumber<Health>;

/// A pokemon owned by a player.
/// This can be (de)serialized and does not borrow values.
pub type SavedPokemon = OwnablePokemon<PokemonId, SavedMoveSet, ItemId, Option<Health>>;
/// A pokemon owned by a player.
/// This struct has borrowed values from multiple [Dex]es.
pub type OwnedPokemon<'d> = OwnablePokemon<&'d Pokemon, OwnedMoveSet<'d>, &'d Item, Health>;

/// The base struct for a pokemon owned by a player.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnablePokemon<P, M, I, H> {
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

impl SavedPokemon {
    /// Generate an owned pokemon.
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
            ivs: ivs.unwrap_or_else(|| Stats::random_iv(random)),
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
}

impl<'d> SavedPokemon {
    /// Initialize this owned pokemon struct into an [OwnedPokemon] so it can perform more functions.
    pub fn init<R: Rng>(
        self,
        random: &mut R,
        pokedex: &'d dyn Dex<Pokemon>,
        movedex: &'d dyn Dex<Move>,
        itemdex: &'d dyn Dex<Item>,
    ) -> Option<OwnedPokemon<'d>> {
        let pokemon = pokedex.try_get(&self.pokemon)?;
        let hp = self
            .hp
            .unwrap_or_else(|| pokemon.stat(&self.ivs, &self.evs, self.level, StatType::Health));
        let mut moves = self.moves.init(movedex)?;
        if moves.is_empty() {
            for m in pokemon.moves_at(1..=self.level).take(4) {
                moves.add(None, m);
            }
        }
        let item = self.item.map(|ref id| itemdex.try_get(id)).flatten();
        let gender = self.gender.or_else(|| pokemon.generate_gender(random));
        Some(OwnedPokemon {
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

impl<'d> OwnedPokemon<'d> {
    /// Get the name of this pokemon.
    /// Returns the nickname or the pokemon's name.
    pub fn name(&self) -> &str {
        self.nickname
            .as_deref()
            .unwrap_or_else(|| self.pokemon.name())
    }

    /// Get the current [Health] of this pokemon.
    pub fn hp(&self) -> Health {
        self.hp
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
        self.pokemon.stat(&self.ivs, &self.evs, self.level, stat)
    }

    /// Heal this pokemon with an optional amount of [Health] and restore all its move's [PP] by an optional amount.
    pub fn heal(&mut self, hp: Option<Health>, pp: Option<PP>) {
        self.heal_hp(hp);
        self.moves.iter_mut().for_each(|o| o.restore(pp));
    }

    /// Heal this pokemon with an optional amount of [Health].
    pub fn heal_hp(&mut self, amount: Option<Health>) {
        let max = self.max_hp();
        self.hp = amount.unwrap_or(max).min(max);
    }

    /// Has the pokemon fainted.
    pub fn fainted(&self) -> bool {
        self.hp == 0
    }

    /// Get this pokemon's moves at its current [Level].
    pub fn moves_at_level(&self) -> impl Iterator<Item = &MoveId> + '_ {
        self.pokemon.moves_at_level(self.level)
    }

    /// Add [Experience] to this pokemon, and also handle level ups.
    pub fn add_exp(&mut self, experience: Experience) -> impl Iterator<Item = &MoveId> + '_ {
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

    /// Get the [Experience] from this pokemon at its current [Level].
    pub fn exp_from(&self) -> Experience {
        self.pokemon.exp_from(self.level)
    }

    /// Handle leveling up.
    pub fn on_level_up(&mut self, previous: Level) -> impl Iterator<Item = &MoveId> + '_ {
        // Get the moves the pokemon learns at the level it just gained.

        let mut moves = self.pokemon.moves_at(previous..self.level).into_iter();

        // Add moves if the player's pokemon does not have a full set of moves.

        while !self.moves.is_full() {
            match moves.next() {
                Some(id) => {
                    self.moves.add(None, id);
                }
                None => break,
            }
        }

        moves
    }

    /// Try to use an [Item] and return true if it succeeds.
    /// This function is incomplete and may change.
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
            ItemUsageKind::Script | ItemUsageKind::Pokeball | ItemUsageKind::None => return false,
        }
        true
    }

    /// Try to use the current [Item] the pokemon is holding.
    /// This function is incomplete and due to change.
    /// !!! Always uses the held item.
    pub fn use_held_item(&mut self) -> bool {
        match self.item.take() {
            Some(item) => self.try_use_item(&item),
            None => false,
        }
    }
}

impl<'d> Uninitializable for OwnedPokemon<'d> {
    type Output = SavedPokemon;

    fn uninit(self) -> Self::Output {
        Self::Output {
            pokemon: self.pokemon.id,
            level: self.level,
            nickname: self.nickname,
            gender: self.gender,
            moves: self.moves.uninit(),
            hp: Some(self.hp),
            item: self.item.map(|item| item.id),
            ailment: self.ailment,
            ivs: self.ivs,
            evs: self.evs,
            experience: self.experience,
            friendship: self.friendship,
        }
    }
}
