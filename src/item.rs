use serde::{Deserialize, Serialize};

use tinystr::TinyStr16;

use crate::{
    id::{Dex, Identifiable, IdentifiableRef},
    item::usage::ItemUsage,
};

pub mod bag;
pub mod usage;

mod stack;
pub use stack::*;

pub type ItemId = <Item as Identifiable>::Id;
pub type StackSize = u16;

pub type Itemdex = Dex<Item>;

pub type ItemRef<'a> = IdentifiableRef<'a, Item>;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Item {
    pub id: ItemId,

    pub name: String,
    pub description: Vec<String>,

    #[serde(default)]
    pub category: ItemCategory,

    #[serde(default = "Item::default_stack_size")]
    pub stack_size: StackSize,

    #[serde(default)]
    pub usage: ItemUsage,
}

impl Identifiable for Item {
    type Id = TinyStr16;

    const UNKNOWN: Self::Id = crate::id::UNKNOWN_ID;

    fn id(&self) -> &Self::Id {
        &self.id
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