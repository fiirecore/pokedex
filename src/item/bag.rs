use core::ops::Deref;

use crate::{
    item::{Item, ItemId, ItemStack, SavedItemStack},
    Dex, Initializable, Uninitializable,
};

pub type SavedBag = Vec<SavedItemStack>;

impl<'d, O: Deref<Target = Item>> Initializable<'d, Item, O> for SavedBag {
    type Output = OwnedBag<O>;

    fn init(self, dex: &'d dyn Dex<'d, Item, O>) -> Option<Self::Output> {
        Some(Self::Output {
            items: self.into_iter().flat_map(|i| i.init(dex)).collect()
        })
    }
}

pub struct OwnedBag<I: Deref<Target = Item>> {
    pub items: Vec<ItemStack<I>>,
}

impl<'d, I: Deref<Target = Item>> OwnedBag<I> {
    pub fn position(&self, id: &ItemId) -> Option<usize> {
        self.items.iter().position(|stack| &stack.item.id == id)
    }

    pub fn use_item(&mut self, id: &ItemId) -> bool {
        self.position(id)
            .map(|index| self.items[index].try_use())
            .unwrap_or_default()
    }

    /// Adds an item stack to the bag. Returns extra items if bag is filled.
    pub fn add_item(&mut self, stack: ItemStack<I>) -> Option<ItemStack<I>> {
        match self.position(&stack.item.id) {
            Some(pos) => self.items[pos].add(stack),
            None => {
                self.items.push(stack);
                None
            }
        }
    }
}

impl<I: Deref<Target = Item>> Uninitializable for OwnedBag<I> {
    type Output = Vec<SavedItemStack>;

    fn uninit(self) -> Self::Output {
        self.items.into_iter().map(ItemStack::uninit).collect()
    }
}
