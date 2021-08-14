use core::ops::{Deref, DerefMut};

use super::{OwnedRefMove, Movedex, OwnedMove, MoveId};

pub const MOVESET_LENGTH: usize = 4;

pub type MoveSet<M> = arrayvec::ArrayVec<[M; MOVESET_LENGTH]>;

pub type OwnedMoveSet<M> = MoveSet<OwnedMove<M>>;

type RefSet<'d, U> = MoveSet<OwnedRefMove<'d, U>>;

pub struct MoveRefSet<'d, U> {
    pub movedex: &'d Movedex<U>,
    pub set: RefSet<'d, U>,
}

impl<'d, U> MoveRefSet<'d, U> {
    pub fn new(movedex: &'d Movedex<U>, set: RefSet<'d, U>) -> Self {
        Self {
            movedex,
            set,
        }
    }

    pub fn world_moves(&self) -> impl Iterator<Item = &MoveId> + '_ {
        self.set.iter().filter(|o| o.m.world).map(|o| &o.m.id)
    }

}

impl<'d, U> Deref for MoveRefSet<'d, U> {
    type Target = RefSet<'d, U>;

    fn deref(&self) -> &Self::Target {
        &self.set
    }
}

impl<'d, U> DerefMut for MoveRefSet<'d, U> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.set
    }
}

impl<'d, U> Clone for MoveRefSet<'d, U> {
    fn clone(&self) -> Self {
        Self {
            movedex: self.movedex,
            set: self.set.clone(),
        }
    }
}

impl<'d, U> core::fmt::Debug for MoveRefSet<'d, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        core::fmt::Debug::fmt(&self.set, f)
    }
}