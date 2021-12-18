use core::ops::Deref;

use crate::Identifiable;

/// A Dex is used to hold types with an identifiable value (see [Identifiable]).
pub trait Dex<'d, I: Identifiable, O: Deref<Target = I>> {
    /// Try to get an identifiable value from the Dex.
    fn try_get(&'d self, id: &I::Id) -> Option<O>;

    /// Get the unknown value from the Dex.
    fn unknown(&'d self) -> O;

    /// Get the identifiable value from the Dex, or return the unknown value.
    fn get(&'d self, id: &I::Id) -> O {
        self.try_get(id).unwrap_or_else(|| self.unknown())
    }

    /// Get the length of the Dex.
    fn len(&self) -> usize;
}

pub use defaults::BasicDex;

mod defaults {

    use core::hash::Hash;

    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use std::collections::HashMap;

    use crate::Identifiable;

    use super::Dex;

    fn name<T: ?Sized>() -> &'static str {
        let name = core::any::type_name::<T>();
        name.split("::").last().unwrap_or(name)
    }

    /// Basic Dex implementation using a HashMap.
    #[repr(transparent)]
    #[derive(Debug, Clone)]
    pub struct BasicDex<I: Identifiable>(pub HashMap<I::Id, I>)
    where
        I::Id: Hash + Eq;

    impl<I: Identifiable> BasicDex<I>
    where
        I::Id: Hash + Eq,
    {
        pub fn new(inner: HashMap<I::Id, I>) -> Self {
            Self(inner)
        }

        pub fn insert(&mut self, v: I) -> Option<I>
        where
            I::Id: Clone,
        {
            self.0.insert(v.id().clone(), v)
        }

        pub fn into_inner(self) -> HashMap<I::Id, I> {
            self.0
        }

        pub fn try_get_named(&self, name: &str) -> Option<&I> {
            self.0
                .values()
                .find(|i| i.name().eq_ignore_ascii_case(name))
        }
    }

    impl<'d, I: Identifiable + Send + Sync> Dex<'d, I, &'d I> for BasicDex<I>
    where
        I::Id: Hash + Eq + Send + Sync,
    {
        fn try_get(&'d self, id: &I::Id) -> Option<&'d I> {
            self.0.get(id)
        }

        fn unknown(&'d self) -> &'d I {
            self.try_get(&I::UNKNOWN).unwrap_or_else(|| {
                panic!(
                    "Could not get unknown {} for \"{}\"",
                    name::<I>(),
                    name::<Self>()
                )
            })
        }

        fn len(&self) -> usize {
            self.0.len()
        }
    }

    impl<I: Identifiable> Default for BasicDex<I>
    where
        I::Id: Hash + Eq,
    {
        fn default() -> Self {
            Self(Default::default())
        }
    }

    /// Serialize Dex as a Vec
    impl<I: Identifiable + Serialize> Serialize for BasicDex<I>
    where
        I::Id: Hash + Eq,
    {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            serializer.collect_seq(self.0.values())
        }
    }

    /// Deserialize Dex from a Vec
    impl<'de, I: Identifiable + Deserialize<'de>> Deserialize<'de> for BasicDex<I>
    where
        I::Id: Hash + Eq + Clone,
    {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            Vec::<I>::deserialize(deserializer)
                .map(|i| Self(i.into_iter().map(|i| (i.id().clone(), i)).collect()))
        }
    }
}
