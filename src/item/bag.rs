use core::{ops::Deref};

use alloc::sync::Arc;
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

use crate::{
    item::{Item, ItemId, ItemStack},
    Dex,
};

use super::UserItemStack;
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct Bag<I>(HashMap<ItemId, ItemStack<I>>);

pub type BagData = Bag<ItemId>;
pub type UserBag = Bag<Arc<Item>>;


impl UserBag {

    // /// Adds an item stack to the bag. Returns extra items if bag is filled.
    pub fn insert(&mut self, stack: UserItemStack) {
        match self.0.get_mut(&stack.item.id) {
            Some(bag_stack) => *bag_stack += stack.count,
            None => {
                self.0.insert(stack.item.id.clone(), stack);
            }
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = &UserItemStack> + '_ {
        self.0.values()
    }

    pub fn get(&self, id: &ItemId) -> Option<&UserItemStack> {
        self.0.get(id)
    }

    pub fn get_mut(&mut self, id: &ItemId) -> Option<&mut UserItemStack> {
        self.0.get_mut(id)
    }

    pub fn contains(&self, id: &ItemId) -> bool {
        self.0.contains_key(id)
    }

    pub fn contains_count(&self, id: &ItemId, count: usize) -> bool {
        self.0.get(id).map(|i| i.count >= count).unwrap_or_default()
    }

    /// If the bag has a certain amount of items or more, it will take them.
    pub fn try_take(&mut self, id: &ItemId, count: usize) -> Option<UserItemStack>
    // where
    //     I: Clone,
    {
        self.get_mut(id).and_then(|stack| stack.try_take(count))
    }

    pub fn take(&mut self, id: &ItemId, count: usize) -> Option<UserItemStack>
    // where
    //     I: Clone,
    {
        self.get_mut(id).map(|stack| stack.take(count))
    }

    pub fn use_item(&mut self, id: &ItemId, consume: bool) -> bool {
        self.0
            .get_mut(id)
            .map(|item| item.try_use(consume))
            .unwrap_or_default()
    }
}

impl BagData {
    pub fn init(&self, dex: &Dex<Item>) -> Option<UserBag> {
        Some(Bag(self
            .0
            .iter()
            .flat_map(|(i, stack)| stack.init(dex).map(|stack| (i.clone(), stack)))
            .collect()))
    }
}

impl<I: Deref<Target = Item>> Bag<I> {

    pub fn data(&self) -> BagData {
        Bag(self.0.iter().map(|(id, stack)| (id.clone(), stack.data())).collect())
    }

}

impl Serialize for BagData {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_seq(self.0.values())
    }
}

impl<'de> Deserialize<'de> for BagData {
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

impl From<alloc::vec::Vec<ItemStack<ItemId>>> for BagData {
    fn from(v: alloc::vec::Vec<ItemStack<ItemId>>) -> Self {
        Self(v.into_iter().map(|i| (i.item, i)).collect())
    }
}
