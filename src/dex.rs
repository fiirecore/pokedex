use hashbrown::HashMap;
use serde::{Deserialize, Serialize, Serializer, Deserializer};

use crate::{name, Identifiable, IdRef};

/// A dex, used to hold types with an identifiable value (see [Identifiable]).
#[repr(transparent)]
#[derive(Debug, Clone, Default)]
pub struct Dex<I: Identifiable>(HashMap<I::Id, I>);

impl<I: Identifiable> Dex<I> {
    /// Create a new Dex.
    pub fn new(dex: HashMap<I::Id, I>) -> Self {
        Self(dex)
    }

    /// Get the inner map of a Dex.
    pub fn inner_mut(&mut self) -> &mut HashMap<I::Id, I> {
        &mut self.0
    }

    /// Try to get a value from the Dex.
    pub fn try_get<'a>(&'a self, id: &I::Id) -> Option<IdRef<'a, I>> {
        self.0.get(id).map(IdRef::of)
    }

    /// Get the unknown value from the Dex.
    pub fn unknown<'a>(&'a self) -> IdRef<'a, I> {
        self.try_get(&I::UNKNOWN)
        .unwrap_or_else(|| {
            panic!(
                "Could not get unknown {} for \"{}\"",
                name::<I>(), name::<Self>()
            )
        })
    }

    /// Get the value from the Dex, or return the unknown value.
    pub fn get<'a>(&'a self, id: &I::Id) -> IdRef<'a, I> {
        self.try_get(id)
            .unwrap_or_else(|| self.unknown())
    }

    /// Get the length of the Dex.
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

/// Serialize Dex as a Vec
impl<I: Identifiable + Serialize> Serialize for Dex<I> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_seq(self.0.values())
    }
}

/// Deserialize Dex from a Vec
impl<'de, I: Identifiable + Deserialize<'de>> Deserialize<'de> for Dex<I> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Vec::<I>::deserialize(deserializer).map(|i| Dex(i.into_iter().map(|i| (*i.id(), i)).collect()))
    }
}