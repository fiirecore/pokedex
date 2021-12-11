use core::ops::Deref;
use std::collections::HashMap;

use crate::{
    item::{Item, ItemId, ItemStack, SavedItemStack},
    Dex, Initializable, Uninitializable,
};

pub type SavedBag = Vec<SavedItemStack>;

impl<'d, O: Deref<Target = Item>> Initializable<'d, Item, O> for SavedBag {
    type Output = OwnedBag<O>;

    fn init(self, dex: &'d dyn Dex<'d, Item, O>) -> Option<Self::Output> {
        Some(OwnedBag(
            self.into_iter()
                .flat_map(|stack| stack.init(dex))
                .map(|stack| (stack.item.id, stack))
                .collect(),
        ))
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Default)]
pub struct OwnedBag<I: Deref<Target = Item>>(HashMap<ItemId, ItemStack<I>>);

impl<'d, I: Deref<Target = Item>> OwnedBag<I> {
    pub fn use_item(&mut self, id: &ItemId) -> bool {
        self.0
            .get_mut(id)
            .map(|item| item.try_use())
            .unwrap_or_default()
    }

    /// Adds an item stack to the bag. Returns extra items if bag is filled.
    pub fn add_item(&mut self, stack: ItemStack<I>) -> bool {
        match self.0.get_mut(&stack.item.id) {
            Some(bag_stack) => bag_stack.add(stack.count),
            None => {
                self.0.insert(stack.item.id, stack);
                true
            }
        }
    }
}

impl<I: Deref<Target = Item>> Uninitializable for OwnedBag<I> {
    type Output = Vec<SavedItemStack>;

    fn uninit(self) -> Self::Output {
        self.0.into_values().map(ItemStack::uninit).collect()
    }
}
