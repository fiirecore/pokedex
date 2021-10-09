use core::slice::{Iter, IterMut};

use crate::{
    moves::{Move, PP},
    Identifiable,
};

pub const DEFAULT_SIZE: usize = 4;

pub trait MoveSet {
    const SIZE: usize = DEFAULT_SIZE;

    type Move;

    fn get(&self, index: usize) -> Option<&Self::Move>;

    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Move>;

    fn iter(&self) -> Iter<'_, Self::Move>;

    fn iter_mut(&mut self) -> IterMut<'_, Self::Move>;

    fn is_empty(&self) -> bool;

    fn is_full(&self) -> bool;

    fn len(&self) -> usize;

    fn add_move(&mut self, index: Option<usize>, id: &<Move as Identifiable>::Id);

    fn restore(&mut self, index: Option<usize>, amount: Option<PP>);
}

#[cfg(feature = "move_set_types")]
pub use defaults::{MoveIdSet, MoveRefSet};

#[cfg(feature = "move_set_types")]
mod defaults {
    use core::slice::{Iter, IterMut};

    use serde::{Deserialize, Serialize};

    use crate::{
        moves::{Move, OwnedIdMove, OwnedMove, OwnedRefMove, PP},
        Dex, Identifiable, Initializable, Uninitializable,
    };

    use super::{MoveSet, DEFAULT_SIZE};

    type A<O> = arrayvec::ArrayVec<[O; DEFAULT_SIZE]>;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct MoveIdSet(A<<Self as MoveSet>::Move>);

    impl MoveSet for MoveIdSet {
        type Move = OwnedMove<<Move as Identifiable>::Id, Option<PP>>;

        fn get(&self, index: usize) -> Option<&Self::Move> {
            self.0.get(index)
        }

        fn get_mut(&mut self, index: usize) -> Option<&mut Self::Move> {
            self.0.get_mut(index)
        }

        fn iter(&self) -> Iter<'_, Self::Move> {
            self.0.iter()
        }

        fn iter_mut(&mut self) -> IterMut<'_, Self::Move> {
            self.0.iter_mut()
        }

        fn is_empty(&self) -> bool {
            self.0.is_empty()
        }

        fn is_full(&self) -> bool {
            self.0.is_full()
        }

        fn len(&self) -> usize {
            self.0.len()
        }

        fn add_move(&mut self, index: Option<usize>, id: &<Move as Identifiable>::Id) {
            let m = OwnedMove { m: *id, pp: None };
            match self.0.is_full() {
                true => {
                    if let Some(i) = index.map(|i| self.0.get_mut(i)).flatten() {
                        *i = m
                    }
                }
                false => self.0.push(m),
            }
        }

        fn restore(&mut self, index: Option<usize>, amount: Option<PP>) {
            fn m(o: &mut OwnedIdMove, amount: Option<PP>) {
                match amount {
                    Some(add) => {
                        if let Some(pp) = o.pp.as_mut() {
                            *pp = pp.saturating_add(add);
                        }
                    }
                    None => o.pp = None,
                }
            }

            match index {
                Some(index) => {
                    if let Some(o) = self.0.get_mut(index) {
                        m(o, amount)
                    }
                }
                None => self.iter_mut().for_each(|o| m(o, amount)),
            }
        }
    }

    impl<'d, D: Dex<Move> + 'd> Initializable<'d, D> for MoveIdSet {
        type Identifier = Move;

        type Output = MoveRefSet<'d, D>;

        fn init(self, dex: &'d D) -> Option<Self::Output> {
            let mut moves: A<<Self::Output as MoveSet>::Move> = Default::default();
            for m in self.0.into_iter().map(|o| o.init(dex)) {
                match m {
                    Some(m) => moves.push(m),
                    None => todo!(),
                }
            }
            Some(MoveRefSet(moves, dex))
        }
    }

    impl Default for MoveIdSet {
        fn default() -> Self {
            Self(Default::default())
        }
    }

    #[derive(Debug, Clone)]
    pub struct MoveRefSet<'d, D: Dex<Move>>(A<<Self as MoveSet>::Move>, &'d D);

    impl<'d, D: Dex<Move>> MoveSet for MoveRefSet<'d, D> {
        type Move = OwnedMove<&'d Move, PP>;

        fn get(&self, index: usize) -> Option<&Self::Move> {
            self.0.get(index)
        }

        fn get_mut(&mut self, index: usize) -> Option<&mut Self::Move> {
            self.0.get_mut(index)
        }

        fn iter(&self) -> Iter<'_, Self::Move> {
            self.0.iter()
        }

        fn iter_mut(&mut self) -> IterMut<'_, Self::Move> {
            self.0.iter_mut()
        }

        fn is_empty(&self) -> bool {
            self.0.is_empty()
        }

        fn is_full(&self) -> bool {
            self.0.is_full()
        }

        fn len(&self) -> usize {
            self.0.len()
        }

        fn add_move(&mut self, index: Option<usize>, id: &<Move as Identifiable>::Id) {
            if let Some(m) = self.1.try_get(id) {
                let m = OwnedMove { m, pp: m.pp };
                match self.0.is_full() {
                    true => {
                        if let Some(i) = index.map(|i| self.0.get_mut(i)).flatten() {
                            *i = m
                        }
                    }
                    false => self.0.push(m),
                }
            }
        }

        fn restore(&mut self, index: Option<usize>, amount: Option<PP>) {
            fn m(o: &mut OwnedRefMove<'_>, amount: Option<PP>) {
                match amount {
                    Some(amount) => o.pp = o.pp.saturating_add((o.pp + amount).min(o.m.pp)),
                    None => o.pp = o.m.pp,
                }
            }

            match index {
                Some(index) => {
                    if let Some(o) = self.0.get_mut(index) {
                        m(o, amount)
                    }
                }
                None => self.iter_mut().for_each(|o| m(o, amount)),
            }
        }
    }

    impl<D: Dex<Move>> Uninitializable for MoveRefSet<'_, D> {
        type Output = MoveIdSet;

        fn uninit(self) -> Self::Output {
            MoveIdSet(self.0.into_iter().map(|o| o.uninit()).collect())
        }
    }
}
