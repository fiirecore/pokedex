use serde::{Deserialize, Serialize};
use std::{cmp::Reverse, fmt::Display};

use crate::{
    battle::party::battle::BattlePartyPokemon,
    item::ItemId,
    moves::{
        target::{MoveTargetInstance, MoveTargetLocation},
        Priority,
    },
    pokemon::stat::{BaseStat, BattleStatType, StatType},
};

mod active;
pub mod party;
pub mod view;

pub use active::*;

use self::party::BattleParty;

pub type Active = usize;
pub type PartyIndex = usize;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MovePriority {
    First,
    Second(Reverse<Priority>, Reverse<BaseStat>), // priority, speed <- fix last, make who goes first random
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BattleMove {
    Move(usize, MoveTargetInstance),
    UseItem(ItemId, MoveTargetLocation),
    Switch(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct PokemonIndex<ID: Sized + Copy + Display + PartialEq> {
    pub team: ID,
    pub index: usize,
}

impl<ID: Sized + Copy + Display + PartialEq> Display for PokemonIndex<ID> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} #{}", self.team, self.index)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ActionInstance<ID: Sized + Copy + Display + PartialEq, T> {
    pub pokemon: PokemonIndex<ID>,
    pub action: T,
}

pub type BattleMoveInstance<ID> = ActionInstance<ID, BattleMove>;

type MoveBattleParty<ID> = BattleParty<ID, ActivePokemon, BattlePartyPokemon>;

pub fn move_queue<ID: Sized + Copy + Display + PartialEq>(
    player1: &mut MoveBattleParty<ID>,
    player2: &mut MoveBattleParty<ID>,
) -> Vec<BattleMoveInstance<ID>> {
    use std::collections::BTreeMap;

    fn insert<ID: Sized + Copy + Display + PartialEq>(
        map: &mut BTreeMap<MovePriority, BattleMoveInstance<ID>>,
        party: &mut MoveBattleParty<ID>,
    ) {
        for index in 0..party.active.len() {
            if let Some(pokemon) = party.active.get_mut(index) {
                if let Some(action) = pokemon.use_move() {
                    if let Some(instance) = party.active(index) {
                        let pokemon = PokemonIndex {
                            team: party.id,
                            index,
                        };
                        map.insert(
                            match action {
                                BattleMove::Move(index, ..) => MovePriority::Second(
                                    Reverse(instance.pokemon.moves[index].move_ref.priority),
                                    Reverse(
                                        instance
                                            .pokemon
                                            .base
                                            .get(BattleStatType::Basic(StatType::Speed)),
                                    ),
                                ),
                                _ => MovePriority::First,
                            },
                            BattleMoveInstance { pokemon, action },
                        );
                    }
                }
            }
        }
    }

    let mut map = BTreeMap::new();

    insert(&mut map, player1);
    insert(&mut map, player2);

    map.into_iter().map(|(_, i)| i).collect() // into_values
}
