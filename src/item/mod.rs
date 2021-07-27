use serde::{Deserialize, Serialize};

use deps::str::TinyStr16;
use hashbrown::HashMap;

use crate::id::{Dex, Identifiable, IdentifiableRef};

pub mod bag;
pub mod script;
mod stack;
mod uses;

pub use stack::*;
pub use uses::*;

pub type ItemId = TinyStr16;
pub type StackSize = u16;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Item {
    pub id: ItemId,

    pub name: String,
    pub description: Vec<String>,

    #[serde(default = "default_stack_size")]
    pub stack_size: StackSize,

    #[serde(default, rename = "use")]
    pub usage: ItemUseType,
}

impl Identifiable for Item {
    type Id = ItemId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

pub struct Itemdex;

pub type ItemRef = IdentifiableRef<Itemdex>;

static mut ITEMDEX: Option<HashMap<ItemId, Item>> = None;

impl Dex for Itemdex {
    type Kind = Item;

    const UNKNOWN: ItemId = crate::UNKNOWN_ID;

    fn dex() -> &'static HashMap<ItemId, Self::Kind> {
        unsafe { ITEMDEX.as_ref().unwrap() }
    }

    fn dex_mut() -> &'static mut Option<HashMap<ItemId, Self::Kind>> {
        unsafe { &mut ITEMDEX }
    }
}

pub const fn default_stack_size() -> StackSize {
    999
}
