use core::fmt::{Debug, Display, Formatter, Result as FmtResult};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::ops::Deref;

use super::{Dex, Identifiable};

pub struct IdentifiableRef<D: Dex + ?Sized>(&'static D::Kind);

impl<D: Dex + ?Sized> IdentifiableRef<D> {
    pub fn of(v: &'static D::Kind) -> Self {
        Self(v)
    }
}

impl<D: Dex> Deref for IdentifiableRef<D> {
    type Target = D::Kind;

    fn deref(&self) -> &'static Self::Target {
        &self.0
    }
}

impl<D: Dex> Serialize for IdentifiableRef<D> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.id().serialize(serializer)
    }
}

impl<'de, DEX: Dex> Deserialize<'de> for IdentifiableRef<DEX> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(DEX::get(&<DEX::Kind as Identifiable>::Id::deserialize(deserializer)?))
    }
}

impl<D: Dex> Clone for IdentifiableRef<D> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<D: Dex> Copy for IdentifiableRef<D> {}

impl<D: Dex> Display for IdentifiableRef<D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(self.id(), f)
    }
}

impl<D: Dex> Debug for IdentifiableRef<D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(self.id(), f)
    }
}

impl<D: Dex> PartialEq for IdentifiableRef<D> {
    fn eq(&self, other: &Self) -> bool {
        self.id().eq(other.id())
    }
}

impl<D: Dex> Eq for IdentifiableRef<D> {}
