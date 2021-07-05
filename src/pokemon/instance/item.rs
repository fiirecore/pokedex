use crate::{
    pokemon::{
        instance::PokemonInstance,
    },
    item::{
        script::{
            ItemScript,
            ItemActionKind,
        }
    }
};

impl PokemonInstance {

	// pub fn use_held_item(&mut self) -> bool {
	// 	if let Some(item) = self.item {
	// 		if let Some(conditions) = item.script.conditions.as_ref() {
	// 			for condition in conditions {
	// 				match condition {
	// 				    ItemCondition::BelowHealthPercent(percent) => {
	// 						if (self.current_hp as f32 / self.base.hp as f32) >= *percent {
	// 							return false;
	// 						}
	// 					}
	// 				}
	// 			}
	// 			self.execute_item(item);
	// 			self.item = None;
	// 			true
	// 		} else {
	// 			false
	// 		}
	// 	} else {
	// 		false
	// 	}
	// }

    pub fn execute_item_script(&mut self, script: &ItemScript) { // return result
        for action in &script.actions {
            match action {
                ItemActionKind::CurePokemon(status) => {
                    if let Some(effect) = self.effect {
                        if let Some(status) = status {
                            if &effect.status == status {
                                self.effect = None;
                            }
                        } else {
                            self.effect = None;
                        }
                    }
                }
                ItemActionKind::HealPokemon(hp) => {
                    self.current_hp += (*hp).min(self.max_hp())
                }
            }
        }
	}

}