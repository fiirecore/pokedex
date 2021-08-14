use serde::{Deserialize, Serialize};

use crate::item::{ItemId, ItemRef, StackSize, Itemdex};

pub type ItemIdStack = ItemStack<ItemId>;
pub type ItemRefStack<'a> = ItemStack<ItemRef<'a>>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemStack<I> {
    pub item: I,
    pub count: StackSize,
}

impl<I> ItemStack<I> {
    pub fn new(i: I, count: StackSize) -> Self {
        Self { item: i, count }
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

impl ItemIdStack {

    pub fn init<'d>(self, itemdex: &'d Itemdex) -> Option<ItemRefStack<'d>> {
        Some(ItemRefStack {
            item: itemdex.try_get(&self.item)?,
            count: self.count,
        })
    }

}

impl<'a> ItemRefStack<'a> {
    pub fn add(&mut self, stack: ItemRefStack<'a>) -> Option<ItemRefStack<'a>> {
        self.count = self.count.saturating_add(stack.count);
        let max = self.item.stack_size;
        match self.count > max {
            true => {
                let count = self.count - max;
                self.count = max;
                Some(ItemStack {
                    item: stack.item,
                    count,
                })
            }
            false => None,
        }
    }

    pub fn uninit(self) -> ItemIdStack {
        ItemIdStack {
            item: self.item.id,
            count: self.count,
        }
    }
}
