use serde::{Deserialize, Serialize};

use crate::moves::{MoveId, MoveRef, MoveSet, Movedex, PP};

pub type UninitMove = MoveInstance<MoveId>;
pub type InitMove<'a> = MoveInstance<MoveRef<'a>>;

pub type UninitMoveSet = MoveSet<UninitMove>;
pub type InitMoveSet<'a> = MoveSet<InitMove<'a>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MoveInstance<M> {
    #[serde(rename = "move")]
    pub m: M,
    pub pp: PP,
    // pub decrement: Option<PP>,
}

impl<M> MoveInstance<M> {
    pub fn try_use(&self) -> Option<&M> {
        match self.empty() {
            false => Some(&self.m),
            true => None,
        }
    }

    pub fn decrement(&mut self) {
        self.pp = self.pp.saturating_sub(1);
    }

    pub fn empty(&self) -> bool {
        self.pp == 0
    }
}

impl UninitMove {

    pub fn init<'a>(self, movedex: &'a Movedex) -> Option<InitMove> {
        Some(InitMove {
            m: movedex.try_get(&self.m)?,
            pp: self.pp,
        })
    }
}

impl<'a> InitMove<'a> {
    pub fn new(m: MoveRef<'a>) -> Self {
        Self { pp: m.pp, m }
    }

    pub fn restore(&mut self, amount: Option<PP>) {
        self.pp = amount.unwrap_or(self.m.pp).min(self.m.pp)
    }

}

impl From<MoveId> for UninitMove {
    fn from(id: MoveId) -> Self {
        Self { m: id, pp: 0 }
    }
}

impl<'a> From<InitMove<'a>> for UninitMove {
    fn from(i: InitMove<'a>) -> Self {
        Self {
            m: i.m.id,
            pp: i.pp,
        }
    }
}