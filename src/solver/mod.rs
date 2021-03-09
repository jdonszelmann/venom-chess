use crate::game_engine::board::Board;
use crate::stats::{Stats, StatsEntry};

pub mod random_play;
pub mod minimax;
pub mod alpha_beta;
pub mod alpha_beta_transp;
pub mod move_order;
pub mod quiescence;
pub mod ansi_player;
pub mod fallback_player;
pub mod player;

pub trait Solver {
    /// If a solver sets this to true, it is responsible
    /// for printing the board itself, and it must also print the stats object
    const PRINT_OWN_BOARD: bool = false;

    fn make_move<B: Board>(&mut self, board: B, stats: Stats) -> Option<B> {
        let mut entry = stats.new_entry();
        let res = self.make_move_impl(board, &mut entry);
        stats.finish_entry(entry);
        res
    }

    fn make_move_impl<B: Board>(&mut self, board: B, stats: &mut StatsEntry) -> Option<B>;
}

