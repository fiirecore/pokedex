use serde::{Deserialize, Serialize};

/// The target of the move.
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum MoveTarget {
    Any,
    Ally,
    Allies,
    UserOrAlly,
    UserAndAllies,
    // UserOrAllies,
    User,
    Opponent,
    AllOpponents,
    RandomOpponent,
    AllOtherPokemon,
    AllPokemon,
    None,
}

impl Default for MoveTarget {
    fn default() -> Self {
        Self::None
    }
}