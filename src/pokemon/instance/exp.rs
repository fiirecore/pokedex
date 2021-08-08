use crate::{
    moves::{MoveId, Movedex, InitMove},
    pokemon::{Experience, Level},
};

impl<'a> super::InitPokemon<'a> {
    pub fn add_exp(&mut self, movedex: &'a Movedex, experience: Experience) -> impl Iterator<Item = MoveId> + '_ {
        // add exp to pokemon

        self.experience += experience * 5;

        // level the pokemon up if they reach a certain amount of exp (and then subtract the exp by the maximum for the previous level)

        let gr = self.pokemon.training.growth_rate;

        let previous = self.level;

        while self.experience > gr.max_exp(self.level) {
            self.experience -= gr.max_exp(self.level);
            self.level += 1;
        }

        self.on_level_up(movedex, previous)
    }

    pub fn exp_from(&self) -> Experience {
        self.pokemon.exp_from(self.level)
    }

    pub fn on_level_up(&mut self, movedex: &'a Movedex, previous: Level) -> impl Iterator<Item = MoveId> + '_ {

        // Get the moves the pokemon learns at the level it just gained.

        let mut moves = self.pokemon.moves_at(previous..self.level).into_iter();

        // Add moves if the player's pokemon does not have a full set of moves.

        while !self.moves.is_full() {
            match moves.next() {
                Some(id) => if let Some(m) = movedex.try_get(&id) {
                    self.moves.push(InitMove::new(m))
                },
                None => break,
            }
        }

        moves
    }
}
