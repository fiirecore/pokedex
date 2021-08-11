use crate::item::{
    usage::{ItemAction, ItemCondition, ItemUsageKind},
    Item,
};

impl<'a> super::InitPokemon<'a> {
    pub fn try_use_item(&mut self, item: &Item) -> bool {
        if !item.usage.conditions.iter().any(|c| match c {
            ItemCondition::Fainted => self.fainted(),
        }) {
            return false;
        }
        match &item.usage.kind {
            ItemUsageKind::Actions(actions) => {
                for action in actions {
                    match action {
                        ItemAction::CurePokemon(status) => {
                            if let Some(effect) = &self.ailment {
                                if let Some(status) = status {
                                    if &effect.ailment == status {
                                        self.ailment = None;
                                    }
                                } else {
                                    self.ailment = None;
                                }
                            }
                        }
                        ItemAction::HealPokemon(hp) => {
                            self.heal_hp(Some(*hp));
                        }
                    }
                }
            }
            ItemUsageKind::Script => log::error!("to-do: item script engines"),
            ItemUsageKind::Pokeball | ItemUsageKind::None => return false,
        }
        true
    }
}
