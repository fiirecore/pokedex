use crate::Identifiable;

/// A Dex is used to hold types with an identifiable value (see [Identifiable]).
pub trait Dex<I: Identifiable> {
    type Inner: Sized + Default;

    /// Create a new Dex from an inner type.
    fn new(dex: Self::Inner) -> Self;

    /// Get the inner type of a Dex.
    fn inner_mut(&mut self) -> &mut Self::Inner;

    /// Inserts a value to the Dex.
    fn insert(&mut self, v: I) -> Option<I>;

    /// Try to get an identifiable value from the Dex.
    fn try_get(&self, id: &I::Id) -> Option<&I>;

    /// Get the unknown value from the Dex.
    fn unknown(&self) -> &I;

    /// Get the identifiable value from the Dex, or return the unknown value.
    fn get(&self, id: &I::Id) -> &I {
        self.try_get(id).unwrap_or_else(|| self.unknown())
    }

    /// Get the length of the Dex.
    fn len(&self) -> usize;
}

#[cfg(feature = "dex_types")]
pub use defaults::BasicDex;

#[cfg(feature = "dex_types")]
mod defaults {

    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use crate::Identifiable;

    use super::Dex;

    fn name<T: ?Sized>() -> &'static str {
        let name = core::any::type_name::<T>();
        name.split("::").last().unwrap_or(name)
    }

    /// Basic Dex implementation using hashbrown crate.
    #[repr(transparent)]
    #[derive(Debug)]
    pub struct BasicDex<I: Identifiable>(<Self as Dex<I>>::Inner);

    impl<I: Identifiable> Dex<I> for BasicDex<I> {
        type Inner = hashbrown::HashMap<I::Id, I>;

        fn new(inner: Self::Inner) -> Self {
            Self(inner)
        }

        fn inner_mut(&mut self) -> &mut Self::Inner {
            &mut self.0
        }

        fn insert(&mut self, v: I) -> Option<I> {
            self.0.insert(*v.id(), v)
        }

        fn try_get(&self, id: &I::Id) -> Option<&I> {
            self.0.get(id)
        }

        fn unknown<'a>(&'a self) -> &I {
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

    impl<I: Identifiable> Default for BasicDex<I> {
        fn default() -> Self {
            Self(Default::default())
        }
    }

    /// Serialize Dex as a Vec
    impl<I: Identifiable + Serialize> Serialize for BasicDex<I> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            serializer.collect_seq(self.0.values())
        }
    }

    /// Deserialize Dex from a Vec
    impl<'de, I: Identifiable + Deserialize<'de>> Deserialize<'de> for BasicDex<I> {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            Vec::<I>::deserialize(deserializer)
                .map(|i| Self(i.into_iter().map(|i| (*i.id(), i)).collect()))
        }
    }
}
