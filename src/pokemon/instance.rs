use crate::moves::MoveRef;
use crate::{pokemon::{
		PokemonId,
		Level,
		Pokemon,
		PokemonRef,
		saved::{
			SavedPokemon,
			PokemonData
		},
		data::StatSet,
		InPokedex,
		random::RandomSet,
	},
	moves::{
		instance::MoveInstances,
		serializable::to_instances
	}
};

pub struct PokemonInstance {
	
	pub pokemon: PokemonRef, 
	
	pub data: PokemonData,

	pub moves: MoveInstances,

	pub base: BaseStatSet,

	pub current_hp: u16,
	
}

impl PokemonInstance {

	pub fn new(pokemon: &SavedPokemon) -> Option<Self> {

		crate::pokedex().get(&pokemon.id).map(|pokemon_data| {
			let stats = get_stats(pokemon_data.value(), pokemon.data.ivs, pokemon.data.evs, pokemon.data.level);

			Self {

				data: pokemon.data.clone(),				
				
				moves: pokemon.moves.as_ref().map(|moves| to_instances(moves)).unwrap_or(pokemon_data.moves_from_level(pokemon.data.level)),
	
				base: stats,
				
				current_hp: pokemon.current_hp.unwrap_or(stats.hp),
	
				pokemon: pokemon_data,
				
			}
		})		

	}

	pub fn to_saved(self) -> SavedPokemon {
		SavedPokemon {
		    id: self.pokemon.data.id,
			data: self.data,
		    moves: Some(crate::moves::serializable::from_instances(self.moves)),
		    current_hp: Some(self.current_hp),
		}
	}

	pub fn is_faint(&self) -> bool {
		return self.current_hp == 0;
	}

	pub fn name(&self) -> String {
		self.data.nickname.as_ref().map(|name| name.clone()).unwrap_or(self.pokemon.data.name.to_ascii_uppercase())
	}

	pub fn moves_at_level(&self) -> Vec<MoveRef> {
		let mut moves = Vec::new();
		for pokemon_move in &self.pokemon.moves {
			if pokemon_move.level == self.data.level {
				moves.push(crate::movedex().get(&pokemon_move.move_id).unwrap())
			}
		}
		moves
	}
	
}

impl super::generate::GeneratePokemon for PokemonInstance {

    fn generate(id: PokemonId, min: Level, max: Level, ivs: Option<StatSet>) -> Self {

		let pokemon = crate::pokedex().get(&id).unwrap();

        let level = if min == max {
			max
		} else {
			quad_rand::gen_range(min, max + 1)
		};

		let ivs = ivs.unwrap_or(StatSet::random());
		let evs = StatSet::default();

		let base = get_stats(pokemon.value(), ivs, evs, level);

		Self {

			data: PokemonData {
				nickname: None,
				level: level,
				gender: pokemon.generate_gender(),
				ivs: ivs,
				evs: evs,
				experience: 0,
				friendship: 70,
			},

			moves: pokemon.moves_from_level(level),

			current_hp: base.hp,

			base,
			
			pokemon,
			
		}
    }
}

impl std::fmt::Display for PokemonInstance {

	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Lv. {} {}", self.data.level, self.name())
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