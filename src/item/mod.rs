use serde::{Deserialize, Serialize};

use tinystr::TinyStr16;

use crate::id::{Dex, Identifiable, IdentifiableRef};

pub mod bag;
pub mod script;
mod stack;
mod uses;

pub use stack::*;
pub use uses::*;

pub type ItemId = <Item as Identifiable>::Id;
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
    type Id = TinyStr16;

    const UNKNOWN: Self::Id = crate::id::UNKNOWN_ID;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

pub type Itemdex = Dex<Item>;

pub type ItemRef<'a> = IdentifiableRef<'a, Item>;

pub const fn default_stack_size() -> StackSize {
    999
}
