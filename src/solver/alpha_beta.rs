use crate::game_engine::board::Board;
use rand::seq::IteratorRandom;
use rand::thread_rng;
use crate::game_engine::color::Color::{White, Black};
use crate::solver::Solver;
use crate::solver::move_order::order_moves;
use crate::stats::{StatsEntry, Stats};
use crate::game_engine::board::display::DisplayableBoard;

pub struct AlphaBeta {
    search_depth: u64
}

impl AlphaBeta {
    pub fn new(search_depth: u64) -> Self {
        Self {
            search_depth
        }
    }

    pub fn mini_max_ab(board: &impl Board, depth: u64, mut a: f64, mut b: f64, stats: &mut StatsEntry) -> f64 {
        stats.seen_state();

        if depth == 0 || board.is_terminal().is_some() {
            let terminal = board.is_terminal();
            if terminal.is_some() {
                return if terminal == Some(Black) {
                    f64::NEG_INFINITY
                } else if terminal == Some(White) {
                    f64::INFINITY
                } else {
                    0.0
                };
            }
            return board.heuristic();
        }

        if board.current_player() == White {
            let mut value = f64::NEG_INFINITY;
            for move_res in order_moves(board.all_moves(), board) {

                value = value.max(Self::mini_max_ab(&move_res.board, depth - 1, a, b, stats));
                a = a.max(value);
                if a >= b {
                    break;
                }
            }
            return value;
        } else {
            let mut value = f64::INFINITY;
            for move_res in order_moves(board.all_moves(), board) {

                value = value.min(Self::mini_max_ab(&move_res.board, depth - 1, a, b, stats));
                b = b.min(value);
                if b <= a {
                    break;
                }
            }
            return value;
        }
    }
}

impl Solver for AlphaBeta {
    fn make_move_impl<B: Board>(&mut self, board: DisplayableBoard<B>, stats: &mut StatsEntry) -> Option<DisplayableBoard<B>> {
        let mut rng = thread_rng();

        let mut best_moves = Vec::new();

        if board.current_player() == White {
            let mut best = f64::NEG_INFINITY;
            for move_res in order_moves(board.all_moves(), &board) {
                let score = Self::mini_max_ab(&move_res.board, self.search_depth, f64::NEG_INFINITY, f64::INFINITY, stats);
                if score > best {
                    best = score;
                    best_moves = Vec::new();
                    best_moves.push(move_res.mv)
                } else if score == best {
                    best_moves.push(move_res.mv);
                }
            }

            stats.evaluation(best);
        }

        if board.current_player() == Black {
            let mut best = f64::INFINITY;
            for move_res in order_moves(board.all_moves(), &board) {
                let score = Self::mini_max_ab(&move_res.board, self.search_depth, f64::NEG_INFINITY, f64::INFINITY, stats);
                if score < best {
                    best = score;
                    best_moves = Vec::new();
                    best_moves.push(move_res.mv)
                } else if score == best {
                    best_moves.push(move_res.mv);
                }
            }

            stats.evaluation(best);
        }

        let m = best_moves.into_iter().choose(&mut rng)?;

        Some(board.transition(m))
    }

    fn init_stats(&self, stats_folder: String) -> Stats {
        Stats::new("Minimax with Alpha-Beta pruning", Some(self.search_depth), None, stats_folder, true)
    }
}