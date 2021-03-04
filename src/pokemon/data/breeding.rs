use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Breeding {
	
	// pub groups: Vec<EggGroup>,
	pub gender: Option<u8>, // None = no gender, 0 = 100% female, 100 = 100% male
	// pub cycles: Option<u8>,
	
}