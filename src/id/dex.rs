use hashbrown::HashMap;

use crate::{name, Identifiable, IdentifiableRef as Ref};

pub struct Dex<K: Identifiable>(HashMap<K::Id, K>);

impl<I: Identifiable> Dex<I> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(HashMap::with_capacity(capacity))
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

    #[deprecated(note = "having a panicing function here is bad")]
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

    pub fn set(&mut self, map: HashMap<I::Id, I>) {
        self.0 = map;
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, id: I::Id, i: I) -> Option<I> {
        self.0.insert(id, i)
    }
}
