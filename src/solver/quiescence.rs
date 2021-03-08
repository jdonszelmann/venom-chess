use crate::game_engine::board::Board;
use rand::seq::IteratorRandom;
use rand::thread_rng;
use crate::game_engine::color::Color::{White, Black};
use crate::game_engine::chess_move::Move;
use std::fs::read;
use std::i8;
use crate::solver::Solver;
use crate::solver::move_order::order_moves;

pub struct Quiescence {
    search_depth: u64,
    cur_eval: i32,
    deep_eval: i32,
    normal_nodes: i32,
    deep_nodes: i32,
}

impl Quiescence {
    pub fn new(search_depth: u64) -> Self {
        Self {
            search_depth,
            cur_eval: 0,
            deep_eval: 0,
            normal_nodes: 0,
            deep_nodes: 0,
        }
    }

    pub fn mini_max_ab(&mut self, board: &impl Board, depth: u64, mut a: i32, mut b: i32) -> i32 {
        self.normal_nodes += 1;

        if depth == 0 {
            // return board.get_material_score();
            return Self::quiescense(self, board, a, b);
        }

        if board.is_terminal().is_some() {
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
            for move_res in order_moves(board.all_moves(), board){
                // println!("{}",depth);
                value = value.max(Self::mini_max_ab(self, &move_res.board, depth - 1, a, b));
                a = a.max(value);
                if a >= b {
                    break;
                }
            }
            return value;
        } else {
            let mut value = std::i32::MAX;
            for move_res in order_moves(board.all_moves(), board) {
                value = value.min(Self::mini_max_ab(self, &move_res.board, depth - 1, a, b));
                b = b.min(value);
                if b <= a {
                    break;
                }
            }
            return value;
        }
    }

    pub fn quiescense(&mut self, board: &impl Board, mut a: i32, mut b: i32) -> i32 {
        self.deep_nodes += 1;

        let cur_score = board.get_material_score();

        // println!("{},{},{}",a,b,cur_score);

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
            let moves: Vec<Move> = board.all_moves().into_iter().filter(|a| a.extra.is_capturing()).collect();
            if moves.len()==0{
                return cur_score;
            }
            for move_res in order_moves(moves, board) {
                value = value.max(Self::quiescense(self, &move_res.board, a, b));
                a = a.max(value);
                if a >= b {
                    break;
                }
            }
            return a;
        } else {
            let mut value = std::i32::MAX;
            let moves: Vec<Move> = board.all_moves().into_iter().filter(|a| a.extra.is_capturing()).collect();
            if moves.len()==0{
                return cur_score;
            }
            for move_res in order_moves(moves, board) {
                value = value.min(Self::quiescense(self, &move_res.board, a, b));
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
    fn make_move<B: Board>(&mut self, board: B) -> Option<B> {
        self.normal_nodes = 0;
        self.deep_nodes = 0;

        let mut rng = thread_rng();

        let mut best_moves = Vec::new();

        let mut best = 0;

        if board.current_player() == White {
            best = std::i32::MIN;
            for move_res in order_moves(board.all_moves(), &board) {
                let score = Self::mini_max_ab(self, &move_res.board, self.search_depth, std::i32::MIN, std::i32::MAX);
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
            best = std::i32::MAX;
            for move_res in order_moves(board.all_moves(), &board) {
                let score = Self::mini_max_ab(self, &move_res.board, self.search_depth, i32::MIN, i32::MAX);
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
        self.deep_eval = best;
        self.cur_eval = new_state.get_material_score();
        Some(new_state)
    }

    fn stats(&self) -> String {
        format!("EVAL: {}, {}. EXP: {}, {}.", self.cur_eval, self.deep_eval, self.normal_nodes, self.deep_nodes-self.normal_nodes)
    }
}