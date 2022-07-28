use serde::{Deserialize, Serialize};
use alloc::sync::Arc;

use crate::{
    moves::{Move, MoveId, PP},
    Dex, Identifiable,
};

pub type SavedMove = OwnableMove<MoveId, Option<PP>>;
pub type OwnedMove = OwnableMove<Arc<Move>, PP>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnableMove<M, P>(pub M, pub P);

impl SavedMove {
    pub fn init(
        self,
        dex: &Dex<Move>,
    ) -> Option<OwnedMove> {
        dex.try_get(&self.0).cloned().map(OwnedMove::from)
    }
}

impl OwnedMove {
    pub fn uninit(self) -> SavedMove {
        OwnableMove(*(*self.0).id(), Some(self.1))
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
    pub fn restore(&mut self, amount: Option<PP>) {
        match amount {
            Some(by) => {
                if let Some(pp) = self.1.as_mut() {
                    *pp = pp.saturating_add(by);
                }
            }
            None => self.1 = None,
        }
    }
}

impl OwnedMove {
    pub fn restore(&mut self, amount: Option<PP>) {
        let max = self.0.pp;
        self.1 = amount.unwrap_or(max).min(max)
    }
}

// impl MoveView for SavedMove {
//     fn id(&self) -> MoveId {
//         self.0
//     }

//     fn is_empty(&self) -> bool {
//         self.1 == Some(0)
//     }
// }

impl OwnedMove {
    pub fn id(&self) -> &MoveId {
        &self.0.id
    }

    pub fn is_empty(&self) -> bool {
        self.1 == 0
    }
}

impl From<MoveId> for SavedMove {
    fn from(id: MoveId) -> Self {
        Self(id, None)
    }
}

impl From<Arc<Move>> for OwnedMove {
    fn from(m: Arc<Move>) -> Self {
        let pp = m.pp;
        Self(m, pp)
    }
}
