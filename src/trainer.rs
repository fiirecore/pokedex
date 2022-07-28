use alloc::vec::Vec;

use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    item::{
        bag::{InitBag, SavedBag},
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

pub type SavedTrainer = Trainer<SavedPokemon, SavedBag>;
pub type InitTrainer = Trainer<OwnedPokemon, InitBag>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Trainer<P, B> {
    pub party: Party<P>,
    pub bag: B,
    pub money: Money,
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
