//! Items
//!
//! This module is incomplete and due to change.

use alloc::string::String;

use serde::{Deserialize, Serialize};

use crate::Identifiable;

pub mod bag;

mod stack;
pub use stack::*;

type IdInner = tinystr::TinyStr16;

/// An identifier for items.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(transparent)]
pub struct ItemId(pub IdInner);

pub type Price = crate::Money;

/// An item.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Item {
    pub id: <Self as Identifiable>::Id,

    pub name: String,
    pub description: String,

    #[serde(default)]
    pub category: ItemCategory,

    pub price: Price,

    #[serde(default)]
    pub stackable: Stackable,
}

// impl Identifier<Item> for ItemId {
//     fn as_id(&self) -> &<Item as Identifiable>::Id {
//         self
//     }
// }

impl Identifiable for Item {
    type Id = ItemId;

    const UNKNOWN: Self::Id = ItemId(crate::UNKNOWN_ID);

    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum ItemCategory {
    Items,
    KeyItems,
    Pokeballs,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Stackable {
    Singular,
    Stackable(u16),
}

impl Default for ItemId {
    fn default() -> Self {
        Item::UNKNOWN
    }
}

impl ItemId {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl From<IdInner> for ItemId {
    fn from(inner: IdInner) -> Self {
        Self(inner)
    }
}

impl core::str::FromStr for ItemId {
    type Err = tinystr::TinyStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(Self)
    }
}

impl core::fmt::Display for ItemId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "#{}", self.0)
    }
}

impl Default for Stackable {
    fn default() -> Self {
        Self::Stackable(999)
    }
}

impl Default for ItemCategory {
    fn default() -> Self {
        Self::Items
    }
}
