use serde::{Deserialize, Serialize};

use crate::{pokemon::Health, ailment::Ailment};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ItemUsage {
    #[serde(default)]
    pub conditions: Vec<ItemCondition>,
    #[serde(rename = "type")]
    pub kind: ItemUsageKind,
    #[serde(default = "t")]
    pub consume: bool,
}

const fn t() -> bool {
    true
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum ItemUsageKind {
    Actions(Vec<ItemAction>),
    Script,
    Pokeball,
    None,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum ItemCondition {
    Fainted,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum ItemAction {
    CurePokemon(Option<Ailment>),
    HealPokemon(Health),
}

impl Default for ItemUsageKind {
    fn default() -> Self {
        Self::None
    }
}