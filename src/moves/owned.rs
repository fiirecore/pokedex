use serde::{Deserialize, Serialize};

use crate::{Dex, Initializable, Uninitializable, moves::{Move, MoveId, PP}};

pub type OwnedIdMove = OwnedMove<MoveId, Option<PP>>;
pub type OwnedRefMove<'d> = OwnedMove<&'d Move, PP>;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnedMove<M, PP> {
    #[serde(rename = "move")]
    pub m: M,
    pub pp: PP,
    // pub decrement: Option<PP>,
}

impl<'d, D: Dex<Move> + 'd> Initializable<'d, D> for OwnedIdMove {

    type Output = OwnedRefMove<'d>;

    type Identifier = Move;

    fn init(self, dex: &'d D) -> Option<Self::Output> {
        dex.try_get(&self.m).map(|m| Self::Output { pp: m.pp, m })
    }
}

impl<'d> Uninitializable for OwnedRefMove<'d> {

    type Output = OwnedIdMove;

    fn uninit(self) -> Self::Output {
        Self::Output {
            m: self.m.id,
            pp: Some(self.pp),
        }
    }
}

impl<'d> OwnedRefMove<'d> {
    pub fn new(m: &'d Move) -> Self {
        Self { pp: m.pp, m }
    }

    pub fn try_use(&self) -> Option<&'d Move> {
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
}

impl From<MoveId> for OwnedIdMove {
    fn from(id: MoveId) -> Self {
        Self { m: id, pp: None }
    }
}
