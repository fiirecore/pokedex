use serde::{Deserialize, Serialize};
use alloc::sync::Arc;

use crate::{
    moves::{Move, MoveId, PP},
    Dex, Identifiable,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedMove {
    pub id: MoveId,
    #[serde(default)]
    pub pp: Option<PP>,
}

#[derive(Debug, Clone)]
pub struct OwnedMove {
    pub m: Arc<Move>,
    pub pp: PP,
}

impl SavedMove {

    pub fn id(&self) -> &MoveId {
        &self.id
    }

    pub fn restore(&mut self, amount: Option<PP>) {
        match amount {
            Some(by) => {
                if let Some(pp) = self.pp.as_mut() {
                    *pp = pp.saturating_add(by);
                }
            }
            None => self.pp = None,
        }
    }

    pub fn init(
        self,
        dex: &Dex<Move>,
    ) -> Option<OwnedMove> {
        dex.try_get(&self.id).cloned().map(OwnedMove::from)
    }

    pub fn is_empty(&self) -> bool {
        self.pp == Some(0)
    }
}

impl OwnedMove {

    pub fn id(&self) -> &MoveId {
        &self.m.id
    }

    pub fn is_empty(&self) -> bool {
        self.pp == 0
    }

    pub fn restore(&mut self, amount: Option<PP>) {
        let max = self.m.pp;
        self.pp = amount.unwrap_or(max).min(max)
    }

    pub fn uninit(self) -> SavedMove {
        SavedMove {
            id: self.m.id().clone(), 
            pp: Some(self.pp)
        }
    }
}

impl From<MoveId> for SavedMove {
    fn from(id: MoveId) -> Self {
        Self { id, pp: None }
    }
}

impl From<Arc<Move>> for OwnedMove {
    fn from(m: Arc<Move>) -> Self {
        let pp = m.pp;
        Self { m, pp }
    }
}
