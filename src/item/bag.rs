use crate::{
    item::{Item, ItemId, ItemStack},
    Dex, Initializable, Uninitializable,
};

pub struct Bag<'d> {
    pub itemdex: &'d dyn Dex<Item>,
    pub items: Vec<ItemStack<&'d Item>>,
}

impl<'d> Bag<'d> {
    pub fn init(itemdex: &'d dyn Dex<Item>, items: Vec<ItemStack<ItemId>>) -> Self {
        let items = items.into_iter().flat_map(|s| s.init(itemdex)).collect();
        Self { itemdex, items }
    }

    pub fn position(&self, id: &ItemId) -> Option<usize> {
        self.items.iter().position(|stack| &stack.item.id == id)
    }

    pub fn use_item(&mut self, id: &ItemId) -> bool {
        self.position(id)
            .map(|index| self.items[index].decrement())
            .unwrap_or_default()
    }

    /// Adds an item stack to the bag. Returns extra items if bag is filled.
    pub fn add_item(&mut self, stack: ItemStack<&'d Item>) -> Option<ItemStack<&'d Item>> {
        match self.position(&stack.item.id) {
            Some(pos) => self.items[pos].add(stack),
            None => {
                self.items.push(stack);
                None
            }
        }
    }
}

impl<'d> Uninitializable for Bag<'d> {
    type Output = Vec<ItemStack<ItemId>>;

    fn uninit(self) -> Self::Output {
        self.items.into_iter().map(ItemStack::uninit).collect()
    }
}

impl<'d> Clone for Bag<'d> {
    fn clone(&self) -> Self {
        Self {
            itemdex: self.itemdex,
            items: self.items.clone(),
        }
    }
}
