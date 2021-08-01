use crate::{
    moves::{instance::MoveInstance, MoveId},
    pokemon::{stat::BaseStats, Experience, Level},
};

use super::PokemonInstance;

impl PokemonInstance {
    pub fn add_exp(&mut self, experience: super::Experience) -> impl Iterator<Item = MoveId> + '_ {
        // add exp to pokemon

        self.experience += experience * 5;

        // level the pokemon up if they reach a certain amount of exp (and then subtract the exp by the maximum for the previous level)

        let gr = self.pokemon.training.growth_rate;

        let previous = self.level;

        while self.experience > gr.max_exp(self.level) {
            self.experience -= gr.max_exp(self.level);
            self.level += 1;
        }

        self.on_level_up(previous)
    }

    pub fn exp_from(&self) -> Experience {
        self.pokemon.exp_from(self.level)
    }

    pub fn on_level_up(&mut self, previous: Level) -> impl Iterator<Item = MoveId> + '_ {
        // Updates base stats of pokemon

        self.base = BaseStats::new(&self.pokemon, &self.ivs, &self.evs, self.level);

        // Get the moves the pokemon learns at the level it just gained.

        let mut moves = self.pokemon.moves_at(previous..self.level).into_iter();

        // Add moves if the player's pokemon does not have a full set of moves.

        while !self.moves.is_full() {
            match moves.next() {
                Some(id) => {
                    if let Some(instance) = MoveInstance::new_id(&id) {
                        self.moves.push(instance);
                    }
                }
                None => break,
            }
        }

        moves
    }
}
