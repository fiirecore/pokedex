use serde::{Deserialize, Serialize};

use crate::moves::{MoveId, MoveRef, Movedex, PP};

pub type OwnedIdMove = OwnedMove<MoveId, Option<PP>>;
pub type OwnedRefMove<'d> = OwnedMove<MoveRef<'d>, PP>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnedMove<M, P> {
    #[serde(rename = "move")]
    pub m: M,
    pub pp: P,
    // pub decrement: Option<PP>,
}

impl OwnedIdMove {

    pub fn init<'d>(self, movedex: &'d Movedex) -> Option<OwnedRefMove> {
        let m = movedex.try_get(&self.m)?;
        Some(OwnedRefMove {
            pp: self.pp.unwrap_or(m.pp),
            m
        })
    }
}

impl<'d> OwnedRefMove<'d> {
    pub fn new(m: MoveRef<'d>) -> Self {
        Self { pp: m.pp, m }
    }

    pub fn try_use(&self) -> Option<&MoveRef<'d>> {
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

    pub fn restore(&mut self, amount: Option<PP>) {
        self.pp = amount.unwrap_or(self.m.pp).min(self.m.pp)
    }

    pub fn uninit(self) -> OwnedIdMove {
        OwnedIdMove {
            m: self.m.id,
            pp: Some(self.pp),
        }
    }

}

impl From<MoveId> for OwnedIdMove {
    fn from(id: MoveId) -> Self {
        Self { m: id, pp: None }
    }
}