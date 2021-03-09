use crate::game_engine::board::Board;
use rand::seq::IteratorRandom;
use rand::thread_rng;
use crate::solver::Solver;
use crate::stats::StatsEntry;

pub struct RandomPlay {
}

impl RandomPlay {
    pub fn new() -> Self {
        Self {}
    }
}

impl Solver for RandomPlay {
    fn make_move_impl<B: Board>(&mut self, board: B, _stats: &mut StatsEntry) -> Option<B> {
        let mut rng = thread_rng();

        let m = board.all_moves().into_iter().choose(&mut rng)?;

        Some(board.transition(m))
    }

}