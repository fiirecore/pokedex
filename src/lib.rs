pub mod battle;
pub mod id;
pub mod item;
pub mod moves;
pub mod pokemon;
pub mod status;
pub mod trainer;
pub mod types;

pub const UNKNOWN_ID: tinystr::TinyStr16 = unsafe { tinystr::TinyStr16::new_unchecked(31093567915781749) };