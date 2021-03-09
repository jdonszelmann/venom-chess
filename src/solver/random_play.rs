use crate::game_engine::board::Board;
use rand::seq::IteratorRandom;
use rand::thread_rng;
use crate::solver::Solver;
use crate::stats::{StatsEntry, Stats};
use crate::game_engine::board::display::DisplayableBoard;

pub struct RandomPlay {
}

impl RandomPlay {
    pub fn new() -> Self {
        Self {}
    }
}

impl Solver for RandomPlay {
    fn make_move_impl<B: Board>(&mut self, board: DisplayableBoard<B>, stats: &mut StatsEntry) -> Option<DisplayableBoard<B>> {
        let mut rng = thread_rng();

        let m = board.all_moves().into_iter().choose(&mut rng)?;

        // I guess we're supposed to do it, but it doesn't make much of a difference really.
        stats.seen_state();

        Some(board.transition(m))
    }

    fn init_stats(&self, stats_folder: String) -> Stats {
        Stats::new("Random Search", None, None, stats_folder, true)
    }
}