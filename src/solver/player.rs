use crate::solver::ansi_player::AnsiPlayer;
use crate::solver::fallback_player::FallbackPlayer;
use crate::solver::Solver;
use crate::game_engine::board::Board;
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crate::stats::{StatsEntry, Stats};
use crate::game_engine::board::display::DisplayableBoard;

pub struct Player {
    ansi: AnsiPlayer,
    fallback: FallbackPlayer,
}

impl Player {
    pub fn new() -> Self {
        Self {
            ansi: AnsiPlayer::new(),
            fallback: FallbackPlayer::new(),
        }
    }
}

impl Solver for Player {
    const PRINT_OWN_BOARD: bool = true;

    fn make_move_impl<B: Board>(&mut self, board: DisplayableBoard<B>, stats: &mut StatsEntry) -> Option<DisplayableBoard<B>> {
        if enable_raw_mode().is_ok() {
            disable_raw_mode().unwrap();
            self.ansi.make_move_impl(board, stats)
        } else {
            println!("using fallback");
            self.fallback.make_move_impl(board, stats)
        }
    }

    fn init_stats(&self, stats_folder: String) -> Stats {
        Stats::new("Human player", None, None, stats_folder, false)
    }
}