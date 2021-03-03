use dashmap::mapref::one::Ref;
use dashmap::DashMap as HashMap;

use pokemon::Pokemon;
use moves::PokemonMove;

pub mod pokemon;
pub mod data;
pub mod types;
pub mod moves;
pub mod instance;
pub mod party;
pub mod texture;

lazy_static::lazy_static! {
	pub static ref POKEDEX: HashMap<PokemonId, Pokemon> = HashMap::new();
	pub static ref MOVEDEX: HashMap<MoveId, PokemonMove> = HashMap::new();
}

pub type PokemonRef = Ref<'static, PokemonId, Pokemon>;

pub type PokemonId = u16;
pub type Level = u8;
pub type Stat = u8;

pub type MoveRef = Ref<'static, MoveId, PokemonMove>;

pub type MoveId = u16;