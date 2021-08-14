use serde::{Deserialize, Serialize};

use crate::moves::{MoveId, MoveRef, Movedex, PP};

pub type OwnedIdMove = OwnedMove<MoveId>;
pub type OwnedRefMove<'d, U> = OwnedMove<MoveRef<'d, U>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnedMove<M> {
    #[serde(rename = "move")]
    pub m: M,
    pub pp: PP,
    // pub decrement: Option<PP>,
}

impl<M> OwnedMove<M> {
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

impl OwnedIdMove {

    pub fn init<'d, U>(self, movedex: &'d Movedex<U>) -> Option<OwnedRefMove<U>> {
        Some(OwnedRefMove {
            m: movedex.try_get(&self.m)?,
            pp: self.pp,
        })
    }
}

impl<'d, U> OwnedRefMove<'d, U> {
    pub fn new(m: MoveRef<'d, U>) -> Self {
        Self { pp: m.pp, m }
    }

    pub fn restore(&mut self, amount: Option<PP>) {
        self.pp = amount.unwrap_or(self.m.pp).min(self.m.pp)
    }

    pub fn uninit(self) -> OwnedIdMove {
        OwnedIdMove {
            m: self.m.id,
            pp: self.pp,
        }
    }

}

impl From<MoveId> for OwnedIdMove {
    fn from(id: MoveId) -> Self {
        Self { m: id, pp: 0 }
    }
}