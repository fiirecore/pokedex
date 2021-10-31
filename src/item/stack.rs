use core::ops::Deref;
use serde::{Deserialize, Serialize};

use crate::{
    item::{Item, ItemId, StackSize},
    Dex, Initializable, Uninitializable,
};

pub type SavedItemStack = ItemStack<ItemId>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemStack<I> {
    pub item: I,
    pub count: StackSize,
}

impl<I> ItemStack<I> {
    pub fn new(i: I, count: StackSize) -> Self {
        Self { item: i, count }
    }
}

impl<I: Deref<Target = Item>> ItemStack<I> {
    pub fn try_use(&mut self) -> bool {
        if self.count > 0 {
            if self.item.should_consume() {
                self.count -= 1;
            }
            true
        } else {
            false
        }
    }

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

impl<'d, O: Deref<Target = Item>> Initializable<'d, Item, O> for SavedItemStack {
    type Output = ItemStack<O>;

    fn init(self, dex: &'d dyn Dex<'d, Item, O>) -> Option<Self::Output> {
        Some(Self::Output {
            item: dex.try_get(&self.item)?,
            count: self.count,
        })
    }
}

impl<I: Deref<Target = Item>> Uninitializable for ItemStack<I> {
    type Output = SavedItemStack;

    fn uninit(self) -> Self::Output {
        Self::Output {
            item: self.item.id,
            count: self.count,
        }
    }
}
