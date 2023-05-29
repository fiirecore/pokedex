use alloc::{sync::Arc, vec::Vec};
use core::ops::{Index, IndexMut};
use serde::{Deserialize, Serialize};

use crate::{
    moves::{
        owned::*,
        Move,
    },
    Dex,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[deprecated(note = "fix sizing")]
pub struct MoveSet<M>(Vec<M>, usize);

#[deprecated]
pub type MoveSetData = MoveSet<UserMoveData>;
#[deprecated]
pub type UserMoveSet = MoveSet<UserMove>;

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

impl MoveSetData {
    pub fn init(&self, dex: &Dex<Move>) -> Result<UserMoveSet, usize> {
        Ok(MoveSet(
            {
                let mut moves = Vec::new();
                for (i, m) in self.0.iter().enumerate() {
                    match m.init(dex) {
                        Some(m) => moves.push(m),
                        None => return Err(i),
                    }
                }
                moves
            },
            self.1,
        ))
    }
}

impl UserMoveSet {
    pub fn data(&self) -> MoveSetData {
        MoveSet(self.0.iter().map(UserMove::data).collect(), self.1)
    }
}

impl UserMoveSet {
    pub fn is_full(&self) -> bool {
        self.0.len() >= self.1
    }

    pub fn add(&mut self, index: Option<usize>, m: Arc<Move>) -> bool {
        let m = UserMove::from(m);
        match self.is_full() {
            true => {
                if let Some(i) = index.and_then(|i| self.0.get_mut(i)) {
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
