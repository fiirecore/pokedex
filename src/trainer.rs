use alloc::vec::Vec;

use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    item::{
        bag::*,
        Item,
    },
    moves::Move,
    pokemon::{
        owned::*,
        party::Party,
        Pokemon,
    },
    Dex, Money,
};

type IdInner = tinystr::TinyAsciiStr<16>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TrainerGroupId(pub IdInner);

pub type TrainerData = Trainer<UserPokemonData, BagData>;
pub type UserTrainer = Trainer<UserPokemon, UserBag>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Trainer<P, B> {
    pub party: Party<P>,
    pub bag: B,
    pub money: Money,
}

impl From<IdInner> for TrainerGroupId {
    fn from(inner: IdInner) -> Self {
        Self(inner)
    }
}

impl core::str::FromStr for TrainerGroupId {
    type Err = tinystr::TinyStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(Self)
    }
}

impl<P, B: Default> Default for Trainer<P, B> {
    fn default() -> Self {
        Self {
            party: Default::default(),
            bag: Default::default(),
            money: Default::default(),
        }
    }
}

impl TrainerData {
    pub fn init(
        self,
        random: &mut impl Rng,
        pokedex: &Dex<Pokemon>,
        movedex: &Dex<Move>,
        itemdex: &Dex<Item>,
    ) -> Option<UserTrainer> {
        Some(UserTrainer {
            party: {
                let mut party = Vec::new();
                for pokemon in self.party {
                    party.push(pokemon.init(pokedex, movedex, itemdex, Some(random)).ok()?);
                }
                party
            },
            bag: self.bag.init(itemdex)?,
            money: self.money,
        })
    }
}

impl UserTrainer
{
    pub fn data(&self) -> TrainerData {
        TrainerData {
            party: self.party.iter().map(|p| p.data()).collect(),
            bag: self.bag.data(),
            money: self.money,
        }
    }
}
