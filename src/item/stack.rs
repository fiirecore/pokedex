use crate::{
    id::Dex,
    item::{ItemId, ItemRef, Itemdex, StackSize},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemStack {
    pub item: ItemRef,
    pub count: StackSize,
}

impl ItemStack {
    pub fn new(id: &ItemId, count: StackSize) -> Self {
        Self {
            item: Itemdex::get(id),
            count,
        }
    }

    pub fn add(&mut self, stack: ItemStack) -> Option<ItemStack> {
        self.count += stack.count;
        let item = &*self.item;
        match self.count > item.stack_size {
            true => {
                let count = self.count - item.stack_size;
                self.count = item.stack_size;
                Some(ItemStack {
                    item: stack.item,
                    count,
                })
            }
            false => None,
        }
    }

    pub fn decrement(&mut self) -> bool {
        if self.count > 0 {
            self.count -= 1;
            true
        } else {
            false
        }
    }
}