use alloc::sync::Arc;
use core::ops::{Add, AddAssign, Deref};
use serde::{Deserialize, Serialize};

use crate::{
    item::{Item, ItemId, Stackable},
    Dex,
};

pub type StackSize = usize;

pub type SavedItemStack = ItemStack<ItemId>;
pub type InitItemStack = ItemStack<Arc<Item>>;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct ItemStack<I> {
    pub item: I,
    pub count: StackSize,
}

impl<I> From<I> for ItemStack<I> {
    fn from(item: I) -> Self {
        Self { item, count: 0 }
    }
}

impl<I: Clone> ItemStack<I> {
    fn take_gt(&mut self, count: StackSize) -> Self {
        self.count -= count;
        Self {
            item: self.item.clone(),
            count,
        }
    }

    pub fn try_take(&mut self, count: StackSize) -> Option<Self> {
        if count > self.count {
            None
        } else {
            Some(self.take_gt(count))
        }
    }

    pub fn take(&mut self, count: StackSize) -> Self {
        if count > self.count {
            let stack = Self {
                item: self.item.clone(),
                count: self.count,
            };
            self.count = 0;
            stack
        } else {
            self.take_gt(count)
        }
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

    pub fn stacks(&self) -> usize {
        match self.item.stackable {
            Stackable::Singular => self.count,
            Stackable::Stackable(size) => self.count / size as usize,
        }
    }
}

impl<I> Add<StackSize> for ItemStack<I> {
    type Output = Self;

    fn add(self, rhs: StackSize) -> Self::Output {
        let count = self.count.saturating_add(rhs);
        Self {
            item: self.item,
            count,
        }
    }
}

impl<I> AddAssign<StackSize> for ItemStack<I> {
    fn add_assign(&mut self, rhs: StackSize) {
        self.count = self.count.saturating_add(rhs);
    }
}

impl SavedItemStack {
    pub fn init(self, dex: &Dex<Item>) -> Option<InitItemStack> {
        Some(ItemStack {
            item: dex.try_get(&self.item)?.clone(),
            count: self.count,
        })
    }
}

impl<I: Deref<Target = Item>> ItemStack<I> {

    pub fn save(&self) -> SavedItemStack {
        ItemStack {
            item: self.item.id,
            count: self.count,
        }
    }
}
