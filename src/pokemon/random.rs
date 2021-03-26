use quad_rand::gen_range;

use firecore_pokedex_lib::pokemon::data::StatSet;

pub trait RandomSet {

    fn random() -> Self;

}

impl RandomSet for StatSet {

    fn random() -> Self {
		Self {
			hp: gen_range(0, 32),
			atk: gen_range(0, 32),
			def: gen_range(0, 32),
			sp_atk: gen_range(0, 32),
			sp_def: gen_range(0, 32),
			speed: gen_range(0, 32),
		}
	}

}