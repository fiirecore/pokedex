use core::ops::Deref;

use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

use crate::{
    item::{Item, ItemId, ItemStack},
    Dex,
};
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct Bag<I>(HashMap<ItemId, ItemStack<I>>);

pub type SavedBag = Bag<ItemId>;

impl<I> Bag<I> {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = &ItemStack<I>> + '_ {
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
    pub fn try_take(&mut self, id: &ItemId, count: usize) -> Option<ItemStack<I>>
    where
        I: Clone,
    {
        self.get_mut(id)
            .map(|stack| stack.try_take(count))
            .flatten()
    }

    pub fn take(&mut self, id: &ItemId, count: usize) -> Option<ItemStack<I>>
    where
        I: Clone,
    {
        self.get_mut(id).map(|stack| stack.take(count))
    }
}

impl SavedBag {
    pub fn insert_saved(&mut self, stack: ItemStack<ItemId>) {
        match self.0.get_mut(&stack.item) {
            Some(bag_stack) => *bag_stack += stack.count,
            None => {
                self.0.insert(stack.item, stack);
            }
        }
    }
}

impl<I: Deref<Target = Item>> Bag<I> {
    /// Adds an item stack to the bag. Returns extra items if bag is filled.
    pub fn insert_init(&mut self, stack: ItemStack<I>) {
        match self.0.get_mut(&stack.item.id) {
            Some(bag_stack) => *bag_stack += stack.count,
            None => {
                self.0.insert(stack.item.id, stack);
            }
        }
    }

    pub fn use_item(&mut self, id: &ItemId) -> bool {
        self.0
            .get_mut(id)
            .map(|item| item.try_use())
            .unwrap_or_default()
    }
}

impl SavedBag {

    pub fn init<I: Deref<Target = Item> + Clone>(self, dex: &impl Dex<Item, Output = I>) -> Option<Bag<I>> {
        Some(Bag(self
            .0
            .into_iter()
            .flat_map(|(i, stack)| stack.init(dex).map(|stack| (i, stack)))
            .collect()))
    }
}

impl<I: Deref<Target = Item>> Bag<I> {
    pub fn uninit(self) -> SavedBag {
        Bag(self.0.into_iter().map(|(id, v)| (id, v.uninit())).collect())
    }
}

impl Serialize for SavedBag {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_seq(self.0.values())
    }
}

impl<'de> Deserialize<'de> for SavedBag {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self(
            alloc::vec::Vec::<ItemStack<ItemId>>::deserialize(deserializer)?
                .into_iter()
                .map(|stack| (stack.item, stack))
                .collect(),
        ))
    }
}

impl<I> Default for Bag<I> {
    fn default() -> Self {
        Self(Default::default())
    }
}
