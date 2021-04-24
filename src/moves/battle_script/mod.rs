use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

// pub enum BattlePokemonActions {

// }

#[derive(Deserialize, Serialize)]
pub struct BattleActionScript {

    pub actions: VecDeque<BattleActionActions>,

}

#[derive(Deserialize, Serialize)]
pub enum BattleActionActions {

    MoveAndReturn(f32),

}