use serde::{Deserialize, Serialize};

use tinystr::TinyStr16;

use crate::{item::usage::ItemUsage, Identifiable};

pub mod bag;
pub mod usage;

mod stack;
pub use stack::*;

/// An identifier for items.
pub type ItemId = TinyStr16;
/// The amount of items in a group of items (a stack).
pub type StackSize = u16;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Item {
    pub id: ItemId,

    pub name: String,
    pub description: String,

    #[serde(default)]
    pub category: ItemCategory,

    #[serde(default = "Item::default_stack_size")]
    pub stack_size: StackSize,

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
    pub const fn default_stack_size() -> StackSize {
        999
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum ItemCategory {
    Items,
    KeyItems,
    Pokeballs,
}

impl Default for ItemCategory {
    fn default() -> Self {
        Self::Items
    }
}
