use firecore_pokedex_lib::pokemon::{
    Pokemon,
    Gender,
    instance::PokemonInstance,
    data::StatSet,
};
use crate::pokemon::PokemonRef;
use crate::moves::instance::MoveInstances;

use super::{InPokedex, random::RandomSet};

pub struct BattlePokemon {
	
	pub pokemon: PokemonRef, 
	
	pub nickname: Option<String>,
	pub level: u8,
    pub gender: Gender,
//	ability: Ability,

	pub moves: MoveInstances,

	pub base: BaseStatSet,

	ivs: StatSet,
	evs: StatSet,

	pub current_hp: u16,

	pub exp: u32,
	
}

impl BattlePokemon {

	pub fn faint(&self) -> bool {
		return self.current_hp == 0;
	}

	pub fn new(pokemon: &PokemonInstance) -> Option<Self> {

		crate::POKEDEX.get(&pokemon.id).map(|pokemon_data| {
			let stats = get_stats(pokemon_data.value(), pokemon.ivs, pokemon.evs, pokemon.level);

			Self {
				
				moves: pokemon_data.moves_from_level(pokemon.level),
				
				nickname: pokemon.nickname.clone(),
				level: pokemon.level,
				gender: pokemon.gender,
				
				ivs: pokemon.ivs,
				
				evs: pokemon.evs,
				
				current_hp: pokemon.current_hp.unwrap_or(stats.hp),
	
				base: stats,
	
				exp: pokemon.exp,
	
				pokemon: pokemon_data,
				
			}
		})		

	}
	
	pub fn generate(pokemon: PokemonRef, min_level: u8, max_level: u8) -> Self {
		let level;
		if min_level == max_level {
			level = max_level;
		} else {
			level = quad_rand::gen_range(min_level, max_level + 1);
		}

		let ivs = StatSet::random();
		let evs = StatSet::default();

		let base = get_stats(pokemon.value(), ivs, evs, level);

		Self {
			
			nickname: None,
			level: level,
            gender: pokemon.generate_gender(),
			
			moves: pokemon.moves_from_level(level),
			
			ivs: ivs,
			evs: evs,

			base: base,

			current_hp: base.hp,
			exp: 0,
			
			pokemon,
			
		}
		
	}

	pub fn to_instance(&self) -> PokemonInstance {
		PokemonInstance {
		    id: self.pokemon.data.number,
			nickname: self.nickname.clone(),
            gender: self.gender,
		    level: self.level,
		    ivs: self.ivs,
		    evs: self.evs,
		    moves: Some(crate::moves::serializable::from_instances(&self.moves)),
		    exp: self.exp,
		    friendship: 70,
		    current_hp: Some(self.current_hp),
		}
	}

	pub fn name(&self) -> &String {
		self.nickname.as_ref().unwrap_or(&self.pokemon.data.name)
	}
	
}

impl std::fmt::Display for BattlePokemon {

	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Lv. {} {}", self.level, &self.pokemon.data.name)
	}
	
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, Default)]
pub struct BaseStatSet {

	pub hp: u16,
	pub atk: u16,
	pub def: u16,
	pub sp_atk: u16,
	pub sp_def: u16,
	pub speed: u16,

}

pub fn get_stats(pokemon: &Pokemon, ivs: StatSet, evs: StatSet, level: u8) -> BaseStatSet {
    BaseStatSet {
		hp: calculate_hp(pokemon.base.hp, ivs.hp, evs.hp, level),
		atk: calculate_stat(pokemon.base.atk, ivs.atk, evs.atk, level),
		def: calculate_stat(pokemon.base.def, ivs.def, evs.def, level),
		sp_atk: calculate_stat(pokemon.base.sp_atk, ivs.sp_atk, evs.sp_atk, level),
		sp_def: calculate_stat(pokemon.base.sp_def, ivs.sp_def, evs.sp_def, level),
		speed: calculate_stat(pokemon.base.speed, ivs.speed, evs.speed, level),
	}
}

pub fn calculate_stat(base_stat: u8, iv_stat: u8, ev_stat: u8, level: u8) -> u16 { //add item check
	let nature = 1.0;
   (((2.0 * base_stat as f64 + iv_stat as f64 + ev_stat as f64) * level as f64 / 100.0 + 5.0).floor() * nature).floor() as u16
}

pub fn calculate_hp(base_hp: u8, iv_hp: u8, ev_hp: u8, level: u8) -> u16 {
   ((2.0 * base_hp as f64 + iv_hp as f64 + ev_hp as f64) * level as f64 / 100.0 + level as f64 + 10.0).floor() as u16
}