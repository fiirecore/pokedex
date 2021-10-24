use serde::{Deserialize, Serialize};

use crate::{
    item::{Item, ItemId, StackSize},
    Dex, Initializable, Uninitializable,
};

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

impl<'d> Initializable<'d, Item> for ItemStack<ItemId> {
    type Output = ItemStack<&'d Item>;

    fn init(self, dex: &'d dyn Dex<Item>) -> Option<Self::Output> {
        Some(Self::Output {
            item: dex.try_get(&self.item)?,
            count: self.count,
        })
    }
}

impl<'d> ItemStack<&'d Item> {
    pub fn add(&mut self, stack: Self) -> Option<Self> {
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
}

impl<'d> Uninitializable for ItemStack<&'d Item> {
    type Output = ItemStack<ItemId>;

    fn uninit(self) -> Self::Output {
        Self::Output {
            item: self.item.id,
            count: self.count,
        }
    }
}
