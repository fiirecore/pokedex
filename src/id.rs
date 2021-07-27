use hashbrown::HashMap;
use serde::{de::DeserializeOwned, Serialize};
use std::{fmt::Display, hash::Hash};

mod reference;
pub use reference::*;

pub trait Identifiable {
    type Id: DeserializeOwned + Serialize + Display + Clone + Copy + Eq + Hash;

    fn id(&self) -> &Self::Id;
}

pub trait Dex {
    // type Map = HashMap<Self::Kind::Id, Self::Kind>;

    type Kind: Identifiable + 'static;

    const UNKNOWN: <Self::Kind as Identifiable>::Id;

    fn dex() -> &'static HashMap<<Self::Kind as Identifiable>::Id, Self::Kind>;

    fn dex_mut() -> &'static mut Option<HashMap<<Self::Kind as Identifiable>::Id, Self::Kind>>;

    fn set(dex: HashMap<<Self::Kind as Identifiable>::Id, Self::Kind>) {
        *Self::dex_mut() = Some(dex)
    }

    fn get(id: &<Self::Kind as Identifiable>::Id) -> IdentifiableRef<Self> {
        Self::try_get(id).unwrap_or_else(|| {
            panic!(
                "Could not get {} with id {} in {}.",
                name::<Self::Kind>(),
                id,
                name::<Self>()
            )
        })
    }

    fn try_get(id: &<Self::Kind as Identifiable>::Id) -> Option<IdentifiableRef<Self>> {
        Self::dex().get(id).map(IdentifiableRef::of)
    }

    fn len() -> usize {
        Self::dex().len()
    }

    fn with_capacity(capacity: usize) -> HashMap<<Self::Kind as Identifiable>::Id, Self::Kind> {
        HashMap::with_capacity(capacity)
    }
}

fn name<T: ?Sized>() -> &'static str {
    let name = std::any::type_name::<T>();
    name.split("::").last().unwrap_or(name)
}
