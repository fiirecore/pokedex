use crate::{
    moves::{
        owned::{OwnedMove, SavedMove},
        Move,
    },
    Dex, Identifiable, Initializable, Uninitializable,
};

pub const DEFAULT_SIZE: usize = 4;

pub type SavedMoveSet = arrayvec::ArrayVec<[SavedMove; DEFAULT_SIZE]>;

impl<'d> Initializable<'d, Move> for SavedMoveSet {

    type Output = OwnedMoveSet<'d>;

    fn init(self, dex: &'d dyn Dex<Move>) -> Option<Self::Output> {
        Some(OwnedMoveSet(dex, self.into_iter().flat_map(|s| s.init(dex)).collect()))
    }
}

pub struct OwnedMoveSet<'d>(&'d dyn Dex<Move>, arrayvec::ArrayVec<[OwnedMove<'d>; DEFAULT_SIZE]>);

impl<'d> OwnedMoveSet<'d> {

    pub fn get(&self, index: usize) -> Option<&OwnedMove<'d>> {
        self.1.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut OwnedMove<'d>> {
        self.1.get_mut(index)
    }

    pub fn iter(&self) -> impl Iterator<Item = &OwnedMove<'d>> {
        self.1.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut OwnedMove<'d>> {
        self.1.iter_mut()
    }

    pub fn is_empty(&self) -> bool {
        self.1.is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.1.is_full()
    }

    pub fn len(&self) -> usize {
        self.1.len()
    }

    pub fn add(&mut self, index: Option<usize>, id: &<Move as Identifiable>::Id) {
        if let Some(m) = self.0.try_get(id) {
            let m = OwnedMove::new(m);
            match self.1.is_full() {
                true => {
                    if let Some(i) = index.map(|i| self.1.get_mut(i)).flatten() {
                        *i = m
                    }
                }
                false => self.1.push(m),
            }
        }
    }

}

impl Uninitializable for OwnedMoveSet<'_> {
    type Output = SavedMoveSet;

    fn uninit(self) -> Self::Output {
        self.1.into_iter().map(|o| o.uninit()).collect()
    }
}

impl core::fmt::Debug for OwnedMoveSet<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.1, f)
    }
}

impl Clone for OwnedMoveSet<'_> {
    fn clone(&self) -> Self {
        Self(self.0, self.1.clone())
    }
}