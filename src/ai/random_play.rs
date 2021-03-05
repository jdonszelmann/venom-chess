use crate::game_engine::board::Board;
use rand::seq::IteratorRandom;
use rand::thread_rng;
use crate::ai::Solver;

pub struct RandomPlay {
}

impl RandomPlay {
    pub fn new() -> Self {
        Self {}
    }
}

impl Solver for RandomPlay {
    fn make_move<B: Board>(&self, board: B) -> Option<B> {
        let mut rng = thread_rng();

        let m = board.all_moves().into_iter().choose(&mut rng)?;

        Some(board.transition(m))
    }
}