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

impl<I: Deref<Target = Item>> OwnedBag<I> {

    /// Adds an item stack to the bag. Returns extra items if bag is filled.
    pub fn insert(&mut self, stack: ItemStack<I>) -> bool {
        match self.0.get_mut(&stack.item.id) {
            Some(bag_stack) => bag_stack.add(stack.count),
            None => {
                self.0.insert(stack.item.id, stack);
                true
            }
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a ItemStack<I>> + 'a {
        self.0.values()
    }

    pub fn get(&self, id: &ItemId) -> Option<&ItemStack<I>> {
        self.0.get(id)
    }

    pub fn get_mut(&mut self, id: &ItemId) -> Option<&mut ItemStack<I>> {
        self.0.get_mut(id)
    }

    pub fn contains(&self, id: &ItemId) -> bool {
        self.0.contains_key(id)
    }

    pub fn contains_count(&self, id: &ItemId, count: usize) -> bool {
        self.0.get(id).map(|i| i.count >= count).unwrap_or_default()
    }

    /// If the bag has a certain amount of items or more, it will take them.
    pub fn try_take(&mut self, id: &ItemId, count: usize) -> Option<ItemStack<I>> where I: Clone {
        self.get_mut(id).map(|stack| stack.try_take(count)).flatten()
    }

    pub fn take(&mut self, id: &ItemId, count: usize) -> Option<ItemStack<I>> where I: Clone {
        self.get_mut(id).map(|stack| stack.take(count))
    }

    pub fn use_item(&mut self, id: &ItemId) -> bool {
        self.0
            .get_mut(id)
            .map(|item| item.try_use())
            .unwrap_or_default()
    }

}

impl<I: Deref<Target = Item>> Uninitializable for OwnedBag<I> {
    type Output = Vec<SavedItemStack>;

    fn uninit(self) -> Self::Output {
        self.0.into_values().map(ItemStack::uninit).collect()
    }
}