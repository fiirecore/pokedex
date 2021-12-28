//! Items
//!
//! This module is incomplete and due to change.

use serde::{Deserialize, Serialize};

use tinystr::TinyStr16;

use crate::{item::usage::ItemUsage, Identifiable};

pub mod bag;
pub mod usage;

mod stack;
pub use stack::*;

/// An identifier for items.
pub type ItemId = TinyStr16;

pub type Price = u32;

/// An item.
#[derive(Debug, Deserialize, Serialize)]
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

    #[serde(default = "Item::consumable_default")]
    pub consume: bool,

    /// Item usage (outside of battle)
    #[serde(default)]
    pub usage: ItemUsage,
}

impl Identifiable for Item {
    type Id = ItemId;

    const UNKNOWN: Self::Id = crate::UNKNOWN_ID;

    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl Item {
    pub const fn should_consume(&self) -> bool {
        self.consume
    }

    const fn consumable_default() -> bool {
        true
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
    Unique,
    Singular,
    Stackable(u16),
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
