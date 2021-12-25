use core::ops::Deref;
use serde::{Deserialize, Serialize};

use crate::{
    moves::{Move, MoveId, PP},
    Dex, Identifiable, Initializable, Uninitializable,
};

pub type SavedMove = OwnableMove<MoveId, Option<PP>>;
pub type OwnedMove<M> = OwnableMove<M, PP>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnableMove<M, P>(pub M, pub P);

impl<'d, O: Deref<Target = Move>> Initializable<'d, Move, O> for SavedMove {
    type Output = OwnedMove<O>;

    fn init(self, dex: &'d dyn Dex<'d, Move, O>) -> Option<Self::Output> {
        dex.try_get(&self.0).map(OwnedMove::from)
    }
}

impl<M: Deref<Target = Move>> Uninitializable for OwnedMove<M> {
    type Output = SavedMove;

    fn uninit(self) -> Self::Output {
        OwnableMove(*self.0.deref().id(), Some(self.1))
    }
}

impl<M, P> OwnableMove<M, P> {
    pub fn pp(&self) -> P
    where
        P: Clone,
    {
        self.1.clone()
    }
}

impl SavedMove {

    pub fn is_empty(&self) -> bool {
        self.1 == Some(0)
    }

    pub fn restore(&mut self, amount: Option<PP>) {
        match amount {
            Some(by) => if let Some(pp) = self.1.as_mut() {
                *pp = pp.saturating_add(by);
            },
            None => self.1 = None,
        }
    }

}

impl<M: Deref<Target = Move>> OwnedMove<M> {
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
