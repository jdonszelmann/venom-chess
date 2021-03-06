use crate::game_engine::board::Board;
use rand::seq::IteratorRandom;
use rand::thread_rng;
use crate::game_engine::color::Color::{White, Black};
use crate::game_engine::chess_move::Move;
use std::fs::read;
use std::i8;
use crate::solver::Solver;
use std::collections::HashMap;
use std::hash::Hash;

pub struct AlphaBetaTransp {
    search_depth: u64,
    table_hits: u64,
}

enum EntryType {
    Exact,
    Lower,
    Upper,
}

struct Entry {
    depth: u64,
    value: i32,
    tp: EntryType,
}

impl AlphaBetaTransp {
    pub fn new(search_depth: u64) -> Self {
        Self {
            search_depth,
            table_hits: 0,
        }
    }

    fn mini_max_ab<B: Board>(&mut self, board: B, depth: u64, mut a: i32, mut b: i32, table: &mut HashMap<B, Entry>) -> i32 {
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

        // Transposition table match
        if let Some(entry) = table.get(&board) {
            if entry.depth >= depth {
                self.table_hits += 1;

                match entry.tp {
                    EntryType::Exact => return entry.value,
                    EntryType::Lower if entry.value > a => {
                        a = entry.value;
                    }
                    EntryType::Upper if entry.value < b => {
                        b = entry.value;
                    }
                    _ => (),
                }

                if a >= b {
                    return entry.value;
                }
            }
        }

        if board.current_player() == White {
            let mut value = std::i32::MIN;
            for m in board.all_moves() {
                let new_board = board.transition(m);
                value = value.max(self.mini_max_ab(new_board, depth - 1, a, b, table));
                a = a.max(value);
                if a >= b {
                    break;
                }
            }

            if value <= a {
                table.insert(board, Entry {
                    depth,
                    value,
                    tp: EntryType::Lower
                });
            } else if value >= b {
                table.insert(board, Entry {
                    depth,
                    value,
                    tp: EntryType::Upper
                });
            } else{
                table.insert(board, Entry {
                    depth,
                    value,
                    tp: EntryType::Exact
                });
            }

            return value;
        } else {
            let mut value = std::i32::MAX;
            for m in board.all_moves() {
                let new_board = board.transition(m);
                value = value.min(self.mini_max_ab(new_board, depth - 1, a, b, table));
                b = b.min(value);
                if b <= a {
                    break;
                }
            }

            if value <= a {
                table.insert(board, Entry {
                    depth,
                    value,
                    tp: EntryType::Lower
                });
            } else if value >= b {
                table.insert(board, Entry {
                    depth,
                    value,
                    tp: EntryType::Upper
                });
            } else{
                table.insert(board, Entry {
                    depth,
                    value,
                    tp: EntryType::Exact
                });
            }

            return value;
        }
    }
}

impl Solver for AlphaBetaTransp {
    fn make_move<B: Board>(&mut self, board: B) -> Option<B> {
        let mut rng = thread_rng();

        let mut transposition_table = HashMap::<B, Entry>::new();


        let mut best_moves = Vec::new();

        if board.current_player() == White {
            let mut best = std::i32::MIN;
            for m in board.all_moves() {
                let new_board = board.transition(m);
                let score = self.mini_max_ab(new_board, self.search_depth, std::i32::MIN, std::i32::MAX, &mut transposition_table);
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
                let score = self.mini_max_ab(new_board, self.search_depth, i32::MIN, i32::MAX, &mut transposition_table);
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