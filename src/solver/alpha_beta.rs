use crate::game_engine::board::Board;
use rand::seq::IteratorRandom;
use rand::thread_rng;
use crate::game_engine::color::Color::{White, Black};
use crate::game_engine::chess_move::Move;
use std::fs::read;
use std::i8;
use crate::solver::Solver;

pub struct AlphaBeta {}

impl AlphaBeta {
    pub fn new() -> Self {
        Self {}
    }

    pub fn mini_max_ab(board: &impl Board, depth: i64, mut a: i8, mut b: i8) -> i8 {
        if depth == 0 || board.is_terminal().is_some() {
            let terminal = board.is_terminal();
            if terminal.is_some() {
                return if terminal == Some(Black) {
                    -127
                } else if terminal == Some(White) {
                    127
                } else {
                    0
                };
            }
            return board.get_material_score();
        }

        if board.current_player() == White {
            let mut value = -std::i8::MAX;
            for m in board.all_moves() {
                let new_board = board.transition(m);
                value = value.max(AlphaBeta::mini_max_ab(&new_board, depth - 1,a,b));
                a = a.max(value);
                if a>=b{
                    break;
                }
            }
            return value;
        } else {
            let mut value = std::i8::MAX;
            for m in board.all_moves() {
                let new_board = board.transition(m);
                value = value.min(AlphaBeta::mini_max_ab(&new_board, depth - 1,a,b));
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

    pub fn make_move<B: Board>(&self, board: B) -> Option<B> {
        let mut rng = thread_rng();

        let mut best_moves = Vec::new();

        if board.current_player() == White {
            let mut best = -std::i8::MAX;
            for m in board.all_moves() {
                let new_board = board.transition(m);
                let score = AlphaBeta::mini_max_ab(&new_board, 4, i8::MIN, i8::MAX);
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
            let mut best = std::i8::MAX;
            for m in board.all_moves() {
                let new_board = board.transition(m);
                let score = AlphaBeta::mini_max_ab(&new_board, 4, i8::MIN, i8::MAX);
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
}