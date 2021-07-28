use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};
use tinystr::TinyStr16;

type Id = TinyStr16;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TrainerId(Id);

impl std::fmt::Display for TrainerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl Default for TrainerId {
    fn default() -> Self {
        Self(crate::UNKNOWN_ID)
    }
}

impl Deref for TrainerId {
    type Target = Id;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TrainerId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TrainerId {
    pub fn new(id: Id) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainerData {
    pub npc_type: TrainerId,
    pub prefix: String,
    pub name: String,
}
