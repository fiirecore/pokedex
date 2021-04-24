use serde::{Deserialize, Serialize};

use crate::moves::persistent::PersistentMove;
use crate::pokemon::status::PokemonStatus;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum MoveAction {

    Damage(u8),
    Status(u8, PokemonStatus), // u8 is 1 - 10, 1 = 10%, 10 = 100%
    Persist(Box<PersistentMove>, bool), // bool = do on current turn

}