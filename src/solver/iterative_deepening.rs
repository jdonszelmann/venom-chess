use crate::game_engine::board::Board;
use rand::seq::IteratorRandom;
use rand::thread_rng;
use crate::game_engine::color::Color::{White, Black};
use crate::solver::Solver;
use crate::solver::move_order::order_moves;
use crate::stats::{StatsEntry, Stats};
use crate::game_engine::board::display::DisplayableBoard;

pub struct IterativeDeepening {
    search_depth: u64
}

impl IterativeDeepening {
    pub fn new(search_depth: u64) -> Self {
        Self {
            search_depth
        }
    }

    pub fn mini_max_ab(board: &impl Board, depth: u64, mut a: i32, mut b: i32, stats: &mut StatsEntry) -> i32 {
        stats.seen_state();

        if depth == 0 || board.is_terminal().is_some() {
            let terminal = board.is_terminal();
            if terminal.is_some() {
                return if terminal == Some(Black) {
                    std::i32::MIN
                } else if terminal == Some(White) {
                    std::i32::MAX
                } else {
                    0
                };
            }
            return board.get_material_score();
        }

        if board.current_player() == White {
            let mut value = std::i32::MIN;
            for move_res in order_moves(board.all_moves(), board) {

                value = value.max(Self::mini_max_ab(&move_res.board, depth - 1, a, b, stats));
                a = a.max(value);
                if a >= b {
                    break;
                }
            }
            return value;
        } else {
            let mut value = std::i32::MAX;
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

impl Solver for IterativeDeepening {
    fn make_move_impl<B: Board>(&mut self, board: DisplayableBoard<B>, stats: &mut StatsEntry) -> Option<DisplayableBoard<B>> {
        let mut rng = thread_rng();

        let mut best_moves = Vec::new();

        if board.current_player() == White {
            let mut best = std::i32::MIN;
            for move_res in order_moves(board.all_moves(), &board) {
                let score = Self::mini_max_ab(&move_res.board, self.search_depth, std::i32::MIN, std::i32::MAX, stats);
                if score > best {
                    best = score;
                    best_moves = Vec::new();
                    best_moves.push(move_res.mv)
                } else if score == best {
                    best_moves.push(move_res.mv);
                }
            }

            stats.evaluation(best as i64);
        }

        if board.current_player() == Black {
            let mut best = std::i32::MAX;
            for move_res in order_moves(board.all_moves(), &board) {
                let score = Self::mini_max_ab(&move_res.board, self.search_depth, i32::MIN, i32::MAX, stats);
                if score < best {
                    best = score;
                    best_moves = Vec::new();
                    best_moves.push(move_res.mv)
                } else if score == best {
                    best_moves.push(move_res.mv);
                }
            }

            stats.evaluation(best as i64);
        }

        let m = best_moves.into_iter().choose(&mut rng)?;

        Some(board.transition(m))
    }

    fn init_stats(&self, stats_folder: String) -> Stats {
        Stats::new("Minimax with Alpha-Beta pruning", Some(self.search_depth), None, stats_folder, true)
    }
}