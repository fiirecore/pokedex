use core::fmt::{Display, Formatter, Result as FmtResult};
use core::ops::Deref;

use super::Identifiable;

pub struct IdentifiableRef<'a, I: Identifiable>(&'a I);

impl<'a, I: Identifiable> IdentifiableRef<'a, I> {
    pub fn of(i: &'a I) -> Self {
        Self(i)
    }
}

impl<'a, I: Identifiable> Deref for IdentifiableRef<'a, I> {
    type Target = I;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a, I: Identifiable> Clone for IdentifiableRef<'a, I> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<'a, I: Identifiable> Copy for IdentifiableRef<'a, I> {}

impl<'a, I: Identifiable> Display for IdentifiableRef<'a, I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(self.id(), f)
    }
}