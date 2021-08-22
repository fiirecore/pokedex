use hashbrown::HashMap;
use serde::{Deserialize, Serialize, Serializer, Deserializer};

use crate::{name, Identifiable, IdentifiableRef as Ref};

#[repr(transparent)]
#[derive(Debug, Clone, Default)]
pub struct Dex<I: Identifiable>(HashMap<I::Id, I>);

impl<I: Identifiable> Dex<I> {
    pub fn new(dex: HashMap<I::Id, I>) -> Self {
        Self(dex)
    }

    pub fn inner_mut(&mut self) -> &mut HashMap<I::Id, I> {
        &mut self.0
    }

    pub fn try_get<'a>(&'a self, id: &I::Id) -> Option<Ref<'a, I>> {
        self.0.get(id).map(Ref::of)
    }

    pub fn unknown<'a>(&'a self) -> Option<Ref<'a, I>> {
        self.try_get(&I::UNKNOWN)
        // .unwrap_or_else(|| {
        //     panic!(
        //         "Could not get unknown {} for dex \"{}\"",
        //         name::<I>(), name::<Self>()
        //     )
        // })
    }

    #[deprecated(note = "having a panicking function here is bad")]
    pub fn get<'a>(&'a self, id: &I::Id) -> Ref<'a, I> {
        self.try_get(id)
            .or_else(|| self.unknown())
            .unwrap_or_else(|| {
                panic!(
                    "Could not get {} with id \"{}\" in {}.",
                    name::<I>(),
                    id,
                    name::<Self>()
                )
            })
    }

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