use alloc::vec::Vec;

use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    item::{
        bag::{OwnedBag, SavedBag},
        Item,
    },
    moves::Move,
    pokemon::{
        owned::{OwnedPokemon, SavedPokemon},
        party::Party,
        Pokemon,
    },
    Dex, Money,
};

type IdInner = tinystr::TinyAsciiStr<16>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TrainerGroupId(pub IdInner);

pub type SavedTrainer = Trainer<SavedPokemon, SavedBag>;
pub type InitTrainer = Trainer<OwnedPokemon, OwnedBag>;

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

impl SavedTrainer {
    pub fn init(
        self,
        random: &mut impl Rng,
        pokedex: &Dex<Pokemon>,
        movedex: &Dex<Move>,
        itemdex: &Dex<Item>,
    ) -> Option<InitTrainer> {
        Some(Trainer {
            party: {
                let mut party = Vec::new();
                for pokemon in self.party {
                    party.push(pokemon.init(random, pokedex, movedex, itemdex)?);
                }
                party
            },
            bag: self.bag.init(itemdex)?,
            money: self.money,
        })
    }
}

impl InitTrainer
{
    pub fn uninit(self) -> SavedTrainer {
        SavedTrainer {
            party: self.party.into_iter().map(|p| p.uninit()).collect(),
            bag: self.bag.uninit(),
            money: self.money,
        }
    }
}
