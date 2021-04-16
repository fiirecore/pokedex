use serde::{Deserialize, Serialize};
use util::tinystr::TinyStr16;
use script::ItemScript;

pub mod script;

pub type ItemId = TinyStr16;
pub type StackSize = u16;

pub type ItemRef = &'static Item;

#[derive(Debug, Deserialize, Serialize)]
pub struct Item {

    pub name: String,
    pub description: String,

    #[serde(default = "default_stack_size")]
    pub stack_size: StackSize,

    pub script: ItemScript,

}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SavedItemStack {
    pub item: ItemId,
    pub count: StackSize,
}

#[derive(Debug)]
pub struct ItemStackInstance {
    pub item: ItemRef,
    pub count: StackSize,
}

pub const fn default_stack_size() -> StackSize {
    999
}