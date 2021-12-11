use core::ops::Deref;
use serde::{Deserialize, Serialize};

use crate::{
    moves::{Move, MoveId, PP},
    Dex, Identifiable, Initializable, Uninitializable,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedMove(pub MoveId, Option<PP>);

#[derive(Debug, Clone, Copy)]
pub struct OwnedMove<M: Deref<Target = Move>>(pub M, pub PP);

impl<'d, O: Deref<Target = Move>> Initializable<'d, Move, O> for SavedMove {
    type Output = OwnedMove<O>;

    fn init(self, dex: &'d dyn Dex<'d, Move, O>) -> Option<Self::Output> {
        dex.try_get(&self.0).map(OwnedMove::from)
    }
}

impl<M: Deref<Target = Move>> Uninitializable for OwnedMove<M> {
    type Output = SavedMove;

    fn uninit(self) -> Self::Output {
        SavedMove(*self.0.deref().id(), Some(self.1))
    }
}

impl<M: Deref<Target = Move>> OwnedMove<M> {

    pub fn pp(&self) -> PP {
        self.1
    }

    pub fn is_empty(&self) -> bool {
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

impl<M: Deref<Target = Move>> From<M> for OwnedMove<M> {
    fn from(m: M) -> Self {
        let pp = m.pp;
        Self(m, pp)
    }
}
