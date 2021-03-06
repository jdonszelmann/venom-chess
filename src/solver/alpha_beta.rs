use crate::game_engine::board::Board;
use rand::seq::IteratorRandom;
use rand::thread_rng;
use crate::game_engine::color::Color::{White, Black};
use crate::game_engine::chess_move::Move;
use std::fs::read;
use std::i8;
use crate::solver::Solver;

pub struct AlphaBeta {
    search_depth: u64
}

impl AlphaBeta {
    pub fn new(search_depth: u64) -> Self {
        Self {
            search_depth
        }
    }

    pub fn mini_max_ab(board: &impl Board, depth: u64, mut a: i32, mut b: i32) -> i32 {
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
            for m in board.all_moves() {
                let new_board = board.transition(m);
                value = value.max(Self::mini_max_ab(&new_board, depth - 1,a,b));
                a = a.max(value);
                if a>=b{
                    break;
                }
            }
            return value;
        } else {
            let mut value = std::i32::MAX;
            for m in board.all_moves() {
                let new_board = board.transition(m);
                value = value.min(Self::mini_max_ab(&new_board, depth - 1,a,b));
                b = b.min(value);
                if b<=a{
                    break;
                }
            }
            return value;
        }
    }
}

impl Solver for AlphaBeta {
    fn make_move<B: Board>(&mut self, board: B) -> Option<B> {
        let mut rng = thread_rng();

        let mut best_moves = Vec::new();

        if board.current_player() == White {
            let mut best = std::i32::MIN;
            for m in board.all_moves() {
                let new_board = board.transition(m);
                let score = Self::mini_max_ab(&new_board, self.search_depth, std::i32::MIN, std::i32::MAX);
                if score > best {
                    best = score;
                    best_moves = Vec::new();
                    best_moves.push(m)
                } else if score == best {
                    best_moves.push(m);
                }
            }
        }

        if board.current_player() == Black {
            let mut best = std::i32::MAX;
            for m in board.all_moves() {
                let new_board = board.transition(m);
                let score = Self::mini_max_ab(&new_board, self.search_depth, i32::MIN, i32::MAX);
                if score < best {
                    best = score;
                    best_moves = Vec::new();
                    best_moves.push(m)
                } else if score == best {
                    best_moves.push(m);
                }
            }
        }

        let m = best_moves.into_iter().choose(&mut rng)?;

        Some(board.transition(m))
    }

    fn stats(&self) -> String {
        "".to_string()
    }
}