use serde::{Deserialize, Serialize};
use core::ops::Deref;

use crate::{
    id::Dex,
    moves::{MoveId, MoveRef, Movedex, PP, MoveSet, Move},
};

pub type MoveInstanceSet = MoveSet<MoveInstance>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MoveInstance {
    #[serde(rename = "move")]
    pub move_ref: MoveRef,
    pub pp: PP,
}

impl MoveInstance {
    pub fn new_id(id: &MoveId) -> Option<Self> {
        Movedex::try_get(&id).map(Self::new)
    }

    pub fn new(move_ref: MoveRef) -> Self {
        Self {
            pp: move_ref.pp,
            move_ref,
        }
    }

    pub fn get(&self) -> Option<MoveRef> {
        (self.pp != 0).then(|| self.move_ref)
    }

    pub fn decrement(&mut self) {
        self.pp = self.pp.saturating_sub(1);
    }

    pub fn empty(&self) -> bool {
        self.pp == 0
    }

    pub fn restore(&mut self) {
        self.pp = self.move_ref.pp;
    }
}

impl Deref for MoveInstance {
    type Target = Move;

    fn deref(&self) -> &Self::Target {
        &self.move_ref
    }
}
