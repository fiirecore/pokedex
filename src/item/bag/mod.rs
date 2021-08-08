use serde::{Deserialize, Serialize};

use crate::item::{ItemRef, ItemStack};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Bag<I: PartialEq> {
    #[serde(default)]
    pub items: Vec<ItemStack<I>>,
}

impl<I: PartialEq> Bag<I> {
    pub fn position(&self, i: &I) -> Option<usize> {
        self.items.iter().position(|stack| &stack.item == i)
    }

    pub fn use_item(&mut self, id: &I) -> bool {
        self.position(id)
            .map(|id| self.items[id].decrement())
            .unwrap_or_default()
    }
}

impl<'a> Bag<ItemRef<'a>> {
    pub fn add_item(&mut self, stack: ItemStack<ItemRef<'a>>) -> Option<ItemStack<ItemRef<'a>>> {
        // returns extra item
        match self
            .items
            .iter()
            .position(|stack2| stack2.item == stack.item)
        {
            Some(pos) => self.items[pos].add(stack),
            None => {
                self.items.push(stack);
                None
            }
        }
    }
}
