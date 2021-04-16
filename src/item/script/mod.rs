use serde::{Serialize, Deserialize};

use crate::pokemon::status::PokemonStatus;

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

    CurePokemon(Option<PokemonStatus>),
    HealPokemon(u16),

}