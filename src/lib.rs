extern crate firecore_dependencies as deps;

pub mod battle;
pub mod id;
pub mod item;
pub mod moves;
pub mod pokemon;
pub mod status;
pub mod trainer;
pub mod types;

pub static RANDOM: deps::random::Random = deps::random::Random::new(deps::random::RandomState::Static(&deps::random::GLOBAL_STATE));

pub const UNKNOWN_ID: deps::str::TinyStr16 = unsafe { deps::str::TinyStr16::new_unchecked(31093567915781749) };