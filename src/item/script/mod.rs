use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemScript {
    
    // pub conditions: Vec<ItemCondition>,
    
    // #[serde(rename = "actions")]
    // original_actions: VecDeque<ItemActionKind>,

    // #[serde(skip)]
    pub actions: Vec<ItemActionKind>, // this should not need to update
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ItemActionKind {

    CurePokemon,
    HealPokemon(u16),

}