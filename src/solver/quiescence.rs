use crate::game_engine::board::Board;
use rand::seq::IteratorRandom;
use rand::thread_rng;
use crate::game_engine::color::Color::{White, Black};
use crate::game_engine::chess_move::Move;
use crate::solver::Solver;
use crate::solver::move_order::order_moves;
use crate::stats::{StatsEntry, Stats};
use crate::game_engine::board::display::DisplayableBoard;

pub struct Quiescence {
    search_depth: u64,
}

impl Quiescence {
    pub fn new(search_depth: u64) -> Self {
        Self {
            search_depth,
        }
    }

    pub fn mini_max_ab(&mut self, board: &impl Board, depth: u64, mut a: f64, mut b: f64, stats: &mut StatsEntry) -> f64 {
        stats.seen_state();

        if depth == 0 {
            // return board.get_material_score();
            stats.custom_int_entry_sub("deep_nodes");
            return Self::quiescense(self, board, a, b, stats);
        }

        if board.is_terminal().is_some() {
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
            for move_res in order_moves(board.all_moves(), board){
                // println!("{}",depth);
                value = value.max(Self::mini_max_ab(self, &move_res.board, depth - 1, a, b, stats));
                a = a.max(value);
                if a >= b {
                    break;
                }
            }
            return value;
        } else {
            let mut value = f64::INFINITY;
            for move_res in order_moves(board.all_moves(), board) {
                value = value.min(Self::mini_max_ab(self, &move_res.board, depth - 1, a, b, stats));
                b = b.min(value);
                if b <= a {
                    break;
                }
            }
            return value;
        }
    }

    pub fn quiescense(&mut self, board: &impl Board, mut a: f64, mut b: f64, stats: &mut StatsEntry) -> f64 {
        stats.custom_int_entry_add("deep_nodes");

        let cur_score = board.heuristic();

        if board.current_player() == White {
            if cur_score >= b {
                return b;
            }
            if a < cur_score {
                a = cur_score;
            }
        } else {
            if cur_score <= b {
                b = cur_score;
            }
            if a > cur_score {
                return a;
            }
        }

        if board.is_terminal().is_some() {
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
            let moves: Vec<Move> = board.all_moves().into_iter().filter(|a| a.extra.is_capturing()).collect();
            if moves.len()==0{
                return cur_score;
            }
            for move_res in order_moves(moves, board) {
                value = value.max(Self::quiescense(self, &move_res.board, a, b, stats));
                a = a.max(value);
                if a >= b {
                    break;
                }
            }
            return a;
        } else {
            let mut value = f64::INFINITY;
            let moves: Vec<Move> = board.all_moves().into_iter().filter(|a| a.extra.is_capturing()).collect();
            if moves.len()==0{
                return cur_score;
            }
            for move_res in order_moves(moves, board) {
                value = value.min(Self::quiescense(self, &move_res.board, a, b, stats));
                b = b.min(value);
                if b <= a {
                    break;
                }
            }
            return b;
        }
    }
}

impl Solver for Quiescence {
    fn make_move_impl<B: Board>(&mut self, board: DisplayableBoard<B>, stats: &mut StatsEntry) -> Option<DisplayableBoard<B>> {
        let mut rng = thread_rng();

        let mut best_moves = Vec::new();

        let mut best = 0.0;

        if board.current_player() == White {
            best = f64::NEG_INFINITY;
            for move_res in order_moves(board.all_moves(), &board) {
                let score = Self::mini_max_ab(self, &move_res.board, self.search_depth, f64::NEG_INFINITY, f64::INFINITY, stats);
                if score > best {
                    best = score;
                    best_moves = Vec::new();
                    best_moves.push(move_res.mv)
                } else if score == best {
                    best_moves.push(move_res.mv);
                }
            }
        }

        if board.current_player() == Black {
            best = f64::INFINITY;
            for move_res in order_moves(board.all_moves(), &board) {
                let score = Self::mini_max_ab(self, &move_res.board, self.search_depth, f64::NEG_INFINITY, f64::INFINITY, stats);
                if score < best {
                    best = score;
                    best_moves = Vec::new();
                    best_moves.push(move_res.mv)
                } else if score == best {
                    best_moves.push(move_res.mv);
                }
            }
        }

        let m = best_moves.into_iter().choose(&mut rng)?;

        let new_state = board.transition(m);
        stats.evaluation(best);
        stats.custom_int_entry("current_evaluation", new_state.get_material_score() as i64);

        Some(new_state)
    }

    fn init_stats(&self, stats_folder: String) -> Stats {
        Stats::new("Quiescence search", Some(self.search_depth), None, stats_folder, true)
    }
}