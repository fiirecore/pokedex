use serde::{Deserialize, Serialize};

use crate::{Dex, Initializable, Uninitializable, moves::{Move, MoveId, PP}};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedMove(pub MoveId, Option<PP>);

#[derive(Debug, Clone, Copy)]
pub struct OwnedMove<'d>(pub &'d Move, PP);

impl<'d, D: Dex<Move>> Initializable<'d, D> for SavedMove {

    type Output = OwnedMove<'d>;

    fn init(self, dex: &'d D) -> Option<Self::Output> {
        dex.try_get(&self.0).map(|m| OwnedMove(m, self.1.unwrap_or(m.pp)))
    }
}

impl<'d> Uninitializable for OwnedMove<'d> {

    type Output = SavedMove;

    fn uninit(self) -> Self::Output {
        SavedMove(self.0.id, Some(self.1))
    }
}

impl<'d> OwnedMove<'d> {
    pub fn new(m: &'d Move) -> Self {
        Self(m, m.pp)
    }

    pub fn try_use(&self) -> Option<&'d Move> {
        match self.empty() {
            false => Some(&self.0),
            true => None,
        }
    }

    pub fn uses(&self) -> PP {
        self.1
    }

    pub fn decrement(&mut self) {
        self.1 = self.1.saturating_sub(1);
    }

    pub fn empty(&self) -> bool {
        self.1 == 0
    }

    pub fn restore(&mut self, amount: Option<PP>) {
        let max = self.0.pp;
        self.1 = amount.unwrap_or(max).min(max)
    }
}

impl From<MoveId> for SavedMove {
    fn from(id: MoveId) -> Self {
        Self(id, None)
    }
}
