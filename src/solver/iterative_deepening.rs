use crate::game_engine::board::Board;
use rand::seq::IteratorRandom;
use rand::thread_rng;
use crate::game_engine::color::Color::{White, Black};
use crate::solver::Solver;
use crate::solver::move_order::order_moves;
use crate::stats::{StatsEntry, Stats};
use crate::game_engine::board::display::DisplayableBoard;
use std::time::{SystemTime, Duration};
use std::ops::Add;
use std::convert::TryInto;

const TIME_DECAY: f64 = 0.99999;

pub struct IterativeDeepening {
}

impl IterativeDeepening {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn mini_max_ab(board: &impl Board, depth: u64, mut a: f64, mut b: f64,deadline:SystemTime, stats: &mut StatsEntry) -> f64 {
        stats.seen_state();

        if SystemTime::now()>deadline{
            return 0.0
        }

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

                value = value.max(TIME_DECAY*Self::mini_max_ab(&move_res.board, depth - 1, a, b, deadline, stats));
                a = a.max(value);
                if a >= b {
                    break;
                }
            }
            return value;
        } else {
            let mut value = f64::INFINITY;
            for move_res in order_moves(board.all_moves(), board) {

                value = value.min(TIME_DECAY*Self::mini_max_ab(&move_res.board, depth - 1, a, b,deadline, stats));
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

        let remaining_time = if board.current_player() == White{
            board.get_clock()[0]
        } else {
            board.get_clock()[1]
        };

        let time_budget = remaining_time / 20;

        let deadline = SystemTime::now()
            .add(time_budget);

        let mut best_moves = Vec::new();
        let mut best_moves_backup = Vec::new();
        let mut best_backup = 0.0;

        let mut search_depth = 0;

        if board.current_player() == White {
            while SystemTime::now()<deadline {
                search_depth += 1;
                best_moves = Vec::new();
                let mut best = f64::NEG_INFINITY;
                for move_res in order_moves(board.all_moves(), &board) {
                    let score = Self::mini_max_ab(&move_res.board, search_depth, f64::NEG_INFINITY, f64::INFINITY,deadline, stats);
                    if score > best {
                        best = score;
                        best_moves = Vec::new();
                        best_moves.push(move_res.mv)
                    } else if score == best {
                        best_moves.push(move_res.mv);
                    }
                }
                if SystemTime::now()<deadline {
                    best_backup = best;
                    best_moves_backup = best_moves.clone();
                }
            }
            stats.evaluation(best_backup);
        }

        if board.current_player() == Black {
            while SystemTime::now()<deadline {
                search_depth += 1;
                best_moves = Vec::new();
                let mut best = f64::INFINITY;
                for move_res in order_moves(board.all_moves(), &board) {
                    let score = Self::mini_max_ab(&move_res.board, search_depth, f64::NEG_INFINITY, f64::INFINITY, deadline, stats);
                    if score < best {
                        best = score;
                        best_moves = Vec::new();
                        best_moves.push(move_res.mv)
                    } else if score == best {
                        best_moves.push(move_res.mv);
                    }
                }
                if SystemTime::now()<deadline {
                    best_backup = best;
                    best_moves_backup = best_moves.clone();
                }
            }
            stats.evaluation(best_backup);
        }

        stats.search_depth(search_depth);

        let m = if best_moves_backup.is_empty(){
            board.all_moves().into_iter().choose(&mut rng)?
        } else{
            best_moves_backup.into_iter().choose(&mut rng)?
        };


        Some(board.transition(m))
    }

    fn init_stats(&self, stats_folder: String) -> Stats {
        Stats::new("Minimax with Alpha-Beta pruning and ID", None,None, stats_folder, true)
    }
}