use serde::{Deserialize, Serialize};

use crate::pokemon::party::Party;

pub mod knowable;
pub mod battle;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BattleParty<ID, A, P> {
    pub id: ID,
    pub name: Option<String>,
    pub active: Vec<A>,
    pub pokemon: Party<P>,
}

impl<ID: Default, A, P> Default for BattleParty<ID, A, P> {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
            active: Default::default(),
            pokemon: Default::default(),
        }
    }
}