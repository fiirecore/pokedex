use core::ops::Deref;

use alloc::vec::Vec;

use rand::Rng;
use serde::{Serialize, Deserialize};

use crate::{pokemon::{owned::{OwnedPokemon, SavedPokemon}, party::Party, Pokemon}, item::{bag::{Bag, SavedBag}, Item}, moves::Move, Dex, Money};

pub type SavedTrainer = Trainer<SavedPokemon, SavedBag>;
pub type InitTrainer<P, M, I> = Trainer<OwnedPokemon<P, M, I>, Bag<I>>;

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
    pub fn init<
        P: Deref<Target = Pokemon> + Clone,
        M: Deref<Target = Move> + Clone,
        I: Deref<Target = Item> + Clone,
    >(
        self,
        random: &mut impl Rng,
        pokedex: &impl Dex<Pokemon, Output = P>,
        movedex: &impl Dex<Move, Output = M>,
        itemdex: &impl Dex<Item, Output = I>,
    ) -> Option<InitTrainer<P, M, I>> {
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

impl<
        P: Deref<Target = Pokemon> + Clone,
        M: Deref<Target = Move> + Clone,
        I: Deref<Target = Item> + Clone,
    > InitTrainer<P, M, I>
{
    pub fn uninit(self) -> SavedTrainer {
        SavedTrainer {
            party: self.party.into_iter().map(|p| p.uninit()).collect(),
            bag: self.bag.uninit(),
            money: self.money,
        }
    }
}