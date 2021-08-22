use core::ops::{Deref, DerefMut};

use super::{OwnedRefMove, Movedex, OwnedMove, MoveId};

pub const MOVESET_LENGTH: usize = 4;

pub type MoveSet<M> = arrayvec::ArrayVec<[M; MOVESET_LENGTH]>;

pub type OwnedMoveSet<M, P> = MoveSet<OwnedMove<M, P>>;

type RefSet<'d> = MoveSet<OwnedRefMove<'d>>;

pub struct MoveRefSet<'d> {
    pub movedex: &'d Movedex,
    pub set: RefSet<'d>,
}

impl<'d> MoveRefSet<'d> {
    pub fn new(movedex: &'d Movedex, set: RefSet<'d>) -> Self {
        Self {
            movedex,
            set,
        }
    }

    pub fn world_moves(&self) -> impl Iterator<Item = &MoveId> + '_ {
        self.set.iter().filter(|o| o.m.world).map(|o| &o.m.id)
    }

}

impl<'d> Deref for MoveRefSet<'d> {
    type Target = RefSet<'d>;

    fn deref(&self) -> &Self::Target {
        &self.set
    }
}

impl<'d> DerefMut for MoveRefSet<'d> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.set
    }
}

impl<'d> Clone for MoveRefSet<'d> {
    fn clone(&self) -> Self {
        Self {
            movedex: self.movedex,
            set: self.set.clone(),
        }
    }
}

impl<'d> core::fmt::Debug for MoveRefSet<'d> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        core::fmt::Debug::fmt(&self.set, f)
    }
}