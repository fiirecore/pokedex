use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Breeding {
    // pub groups: Vec<EggGroup>,
    /// Pokemon gender chance: None = no gender, 0 = 100% female, 7 = 100% male (0-8 scale)
    pub gender: Option<u8>,
    // pub cycles: Option<u8>,
}
