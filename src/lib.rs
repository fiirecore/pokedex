use dashmap::mapref::one::Ref;
use dashmap::DashMap as HashMap;

use pokemon::Pokemon;
use moves::PokemonMove;

pub mod pokemon;
pub mod moves;

pub mod serialized;

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