use alloc::vec::Vec;
use serde::{Serialize, Deserialize};
use core::ops::{Deref, Index, IndexMut};

use crate::{
    moves::{
        owned::{OwnedMove, SavedMove},
        Move,
    },
    Dex,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveSet<M>(Vec<M>, usize);

pub type SavedMoveSet = MoveSet<SavedMove>;
pub type OwnedMoveSet<M> = MoveSet<OwnedMove<M>>;

impl<M> MoveSet<M> {
    pub const DEFAULT_SIZE: usize = 4;

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Option<&M> {
        self.0.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut M> {
        self.0.get_mut(index)
    }

    pub fn iter(&self) -> core::slice::Iter<M> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> core::slice::IterMut<M> {
        self.0.iter_mut()
    }

    pub fn push(&mut self, m: M) {
        self.0.push(m);
    }

}

impl<M> Default for MoveSet<M> {
    fn default() -> Self {
        Self(Default::default(), Self::DEFAULT_SIZE)
    }
}

impl SavedMoveSet {
    pub fn init<M: Deref<Target = Move> + Clone>(
        self,
        dex: &impl Dex<Move, Output = M>,
    ) -> Option<OwnedMoveSet<M>> {
        Some(MoveSet(
            {
                let mut moves = Vec::new();
                for m in self.0 {
                    if let Some(m) = m.init(dex) {
                        moves.push(m);
                    }
                }
                moves
            },
            self.1,
        ))
    }
}

impl<M: Deref<Target = Move>> OwnedMoveSet<M> {
    pub fn uninit(self) -> SavedMoveSet {
        MoveSet(self.0.into_iter().map(OwnedMove::uninit).collect(), self.1)
    }
}

impl<M: Deref<Target = Move>> OwnedMoveSet<M> {
    pub fn is_full(&self) -> bool {
        self.0.len() >= self.1
    }

    pub fn add(&mut self, index: Option<usize>, m: M) -> bool {
        let m = OwnedMove::from(m);
        match self.is_full() {
            true => {
                if let Some(i) = index.map(|i| self.0.get_mut(i)).flatten() {
                    *i = m;
                    true
                } else {
                    false
                }
            }
            false => {
                self.0.push(m);
                true
            }
        }
    }
}

impl<M> Index<usize> for MoveSet<M> {
    type Output = M;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<M> IndexMut<usize> for MoveSet<M> {

    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}