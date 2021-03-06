use crate::game_engine::board::Board;
use rand::seq::IteratorRandom;
use rand::thread_rng;
use crate::game_engine::color::Color::{White, Black};
use crate::game_engine::chess_move::Move;
use std::fs::read;
use std::i8;
use crate::solver::Solver;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use crate::transposition_table::TranspositionTable;
use crate::solver::move_order::order_moves;


pub struct AlphaBetaTransp {
    search_depth: u64,
    table_hits: u64,
    explored: u64,

    transposition_table: TranspositionTable<Entry>,

    last_best: i32,
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
    pub fn new(search_depth: u64, transposition_size: u64) -> Self {
        Self {
            search_depth,
            table_hits: 0,
            explored: 0,
            transposition_table: TranspositionTable::new(transposition_size),
            last_best: 0
        }
    }

    pub fn mini_max_ab<B: Board>(&mut self, board: B, depth: u64, mut a: i32, mut b: i32) -> i32 {
        self.explored += 1;

        let board_hash = board.hash();

        // Transposition table match
        if let Some(entry) = self.transposition_table.get(board_hash) {
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

        if depth == 0 || board.is_terminal().is_some() {
            let terminal = board.is_terminal();

            let value = if terminal.is_some() {
                if terminal == Some(Black) {
                    std::i32::MIN
                } else if terminal == Some(White) {
                    std::i32::MAX
                } else {
                    0
                }
            } else {
                board.get_material_score()
            };

            if value <= a {
                self.transposition_table.insert(board_hash, Entry {
                    depth,
                    value,
                    tp: EntryType::Lower
                });
            } else if value >= b {
                self.transposition_table.insert(board_hash, Entry {
                    depth,
                    value,
                    tp: EntryType::Upper
                });
            } else{
                self.transposition_table.insert(board_hash, Entry {
                    depth,
                    value,
                    tp: EntryType::Exact
                });
            }

            return value;
        }


        let mut value;
        if board.current_player() == White {
            value = std::i32::MIN;
            for move_res in order_moves(board.all_moves(),&board) {
                value = value.max(self.mini_max_ab(move_res.board, depth - 1, a, b));
                a = a.max(value);
                if a >= b {
                    break;
                }
            }

        } else {
            value = std::i32::MAX;
            for move_res in order_moves(board.all_moves(),&board) {
                value = value.min(self.mini_max_ab(move_res.board, depth - 1, a, b));
                b = b.min(value);
                if b <= a {
                    break;
                }
            }
        }

        if value <= a {
            self.transposition_table.insert(board_hash, Entry {
                depth,
                value,
                tp: EntryType::Lower
            });
        } else if value >= b {
            self.transposition_table.insert(board_hash, Entry {
                depth,
                value,
                tp: EntryType::Upper
            });
        } else{
            self.transposition_table.insert(board_hash, Entry {
                depth,
                value,
                tp: EntryType::Exact
            });
        }

        return value
    }
}

impl Solver for AlphaBetaTransp {
    fn make_move<B: Board>(&mut self, board: B) -> Option<B> {
        let mut rng = thread_rng();

        self.table_hits = 0;
        self.explored = 0;

        let mut best_moves = Vec::new();

        let mut best = 0;
        if board.current_player() == White {
            best = i32::MIN;
            for move_res in order_moves(board.all_moves(),&board) {
                let score = self.mini_max_ab(move_res.board, self.search_depth, std::i32::MIN, std::i32::MAX);
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
            best = i32::MAX;
            for move_res in order_moves(board.all_moves(),&board) {
                let score = self.mini_max_ab(move_res.board, self.search_depth, i32::MIN, i32::MAX);
                if score < best {
                    best = score;
                    best_moves = Vec::new();
                    best_moves.push(move_res.mv)
                } else if score == best {
                    best_moves.push(move_res.mv);
                }
            }
        }

        let board_hash = board.hash();
        self.transposition_table.insert(board_hash, Entry {
            depth: self.search_depth,
            value: best,
            tp: EntryType::Exact
        });


        self.last_best = best;

        let m = best_moves.into_iter().choose(&mut rng)?;

        Some(board.transition(m))
    }

    fn stats(&self) -> String {
        format!("tp hits: {:.3}% explored: {} evaluation: {}, tu: {}, tc: {}", self.table_hits as f64 / self.explored as f64, self.explored, self.last_best, self.transposition_table.used, self.transposition_table.collisions)
    }
}