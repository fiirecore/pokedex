use crate::item::{ItemId, ItemIdStack, ItemRefStack, Itemdex};

pub struct Bag<'d> {
    pub itemdex: &'d Itemdex,
    pub items: Vec<ItemRefStack<'d>>,
}

impl<'d> Bag<'d> {
    pub fn init(itemdex: &'d Itemdex, items: Vec<ItemIdStack>) -> Self {
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
    pub fn add_item(&mut self, stack: ItemRefStack<'d>) -> Option<ItemRefStack<'d>> {
        match self.position(&stack.item.id) {
            Some(pos) => self.items[pos].add(stack),
            None => {
                self.items.push(stack);
                None
            }
        }
    }

    pub fn uninit(self) -> Vec<ItemIdStack> {
        self.items.into_iter().map(ItemRefStack::uninit).collect()
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
