//! Miscellaneous values and types used in identification.

use core::fmt::{Debug, Display, Formatter, Result as FmtResult};
use core::ops::Deref;

use tinystr::TinyStr16;

use crate::Identifiable;

/// A TinyStr16 that holds the value "unknown"
pub const UNKNOWN_ID: TinyStr16 = unsafe { TinyStr16::new_unchecked(31093567915781749) };

/// A reference to an identifiable struct, which can also not be serialized/deserialize.
pub struct IdRef<'a, I: Identifiable>(&'a I);

impl<'a, I: Identifiable> IdRef<'a, I> {
    pub fn of(i: &'a I) -> Self {
        Self(i)
    }
    pub fn value(self) -> &'a I {
        self.0
    }
}

impl<'a, I: Identifiable> Deref for IdRef<'a, I> {
    type Target = I;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a, I: Identifiable> Clone for IdRef<'a, I> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, I: Identifiable> Copy for IdRef<'a, I> {}

impl<'a, I: Identifiable> Debug for IdRef<'a, I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(self.id(), f)
    }
}

impl<'a, I: Identifiable> Display for IdRef<'a, I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(self.id(), f)
    }
}
