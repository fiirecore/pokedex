use alloc::sync::Arc;
use core::{hash::Hash, ops::Deref};
use serde::{Serialize, Deserializer, Deserialize, Serializer};

use crate::Identifiable;

/// A Dex is used to hold types with an identifiable value (see [Identifiable]).
#[derive(Debug, Clone)]
pub struct Dex<I: Identifiable>(pub hashbrown::HashMap<I::Id, Arc<I>>);

impl<I: Identifiable> Dex<I> where I::Id: Eq + Hash {
    /// Try to get an identifiable value from the Dex.
    pub fn try_get(&self, id: &I::Id) -> Option<&Arc<I>> {
        self.0.get(id)
    }

    /// Get the unknown value from the Dex.
    pub fn unknown(&self) -> &Arc<I> {
        self.try_get(&I::UNKNOWN).unwrap_or_else(|| {
            panic!(
                "Could not get unknown {} for \"{}\"",
                name::<I>(),
                name::<Self>()
            )
        })
    }

    /// Get the identifiable value from the Dex, or return the unknown value.
    pub fn get(&self, id: &I::Id) -> &Arc<I> {
        self.try_get(id).unwrap_or_else(|| self.unknown())
    }

    /// Get the length of the Dex.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Check if the Dex is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn new(inner: hashbrown::HashMap<I::Id, Arc<I>>) -> Self {
        Self(inner)
    }

    pub fn insert(&mut self, v: I) -> Option<Arc<I>>
    where
        I::Id: Clone,
    {
        self.0.insert(v.id().clone(), Arc::from(v))
    }

    pub fn try_get_named(&self, name: impl AsRef<str>) -> Option<&Arc<I>> {
        let name = name.as_ref();
        self.0
            .values()
            .find(|i| i.name().eq_ignore_ascii_case(name))
    }

    pub fn remove(&mut self, id: &I::Id) -> Option<Arc<I>> {
        self.0.remove(id)
    }
}

fn name<T: ?Sized>() -> &'static str {
    let name = core::any::type_name::<T>();
    name.split("::").last().unwrap_or(name)
}

impl<I: Identifiable> Default for Dex<I>
where
    I::Id: Hash + Eq,
{
    fn default() -> Self {
        Self(Default::default())
    }
}

/// Serialize Dex as a Vec
impl<I: Identifiable + Serialize> Serialize for Dex<I>
where
    I::Id: Hash + Eq,
{
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_seq(self.0.values().map(Deref::deref))
    }
}

/// Deserialize Dex from a Vec
impl<'de, I: Identifiable + Deserialize<'de>>
    Deserialize<'de> for Dex<I>
where
    I::Id: Hash + Eq + Clone,
{
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        alloc::vec::Vec::<I>::deserialize(deserializer).map(|i| {
            Self(
                i.into_iter()
                    .map(|i| (i.id().clone(), Arc::new(i)))
                    .collect(),
            )
        })
    }
}
