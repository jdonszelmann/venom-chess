use crate::game_engine::board::Board;
use rand::seq::IteratorRandom;
use rand::thread_rng;
use crate::game_engine::color::Color::{White, Black};
use crate::solver::Solver;
use crate::transposition_table::TranspositionTable;
use crate::solver::move_order::order_moves;
use crate::stats::{StatsEntry, Stats};
use crate::game_engine::board::display::DisplayableBoard;


pub struct AlphaBetaTransp {
    search_depth: u64,
    transposition_table: TranspositionTable<Entry>,
}

enum EntryType {
    Exact,
    Lower,
    Upper,
}

struct Entry {
    depth: u64,
    value: f64,
    tp: EntryType,
}

impl AlphaBetaTransp {
    pub fn new(search_depth: u64, transposition_size: u64) -> Self {
        Self {
            search_depth,
            transposition_table: TranspositionTable::new(transposition_size),
        }
    }

    pub fn mini_max_ab<B: Board>(&mut self, board: B, depth: u64, mut a: f64, mut b: f64, stats: &mut StatsEntry) -> f64 {
        stats.seen_state();

        let board_hash = board.hash();

        // Transposition table match
        if let Some(entry) = self.transposition_table.get(board_hash) {
            if entry.depth >= depth {
                stats.transposition().hit();

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
                    f64::NEG_INFINITY
                } else if terminal == Some(White) {
                    f64::INFINITY
                } else {
                    0.0
                }
            } else {
                board.heuristic()
            };

            if value <= a {
                self.transposition_table.insert(board_hash, Entry {
                    depth,
                    value,
                    tp: EntryType::Lower
                }, stats.transposition());
            } else if value >= b {
                self.transposition_table.insert(board_hash, Entry {
                    depth,
                    value,
                    tp: EntryType::Upper
                }, stats.transposition());
            } else{
                self.transposition_table.insert(board_hash, Entry {
                    depth,
                    value,
                    tp: EntryType::Exact
                }, stats.transposition());
            }

            return value;
        }


        let mut value;
        if board.current_player() == White {
            value = f64::NEG_INFINITY;
            for move_res in order_moves(board.all_moves(),&board) {
                value = value.max(self.mini_max_ab(move_res.board, depth - 1, a, b, stats));
                a = a.max(value);
                if a >= b {
                    break;
                }
            }

        } else {
            value = f64::INFINITY;
            for move_res in order_moves(board.all_moves(),&board) {
                value = value.min(self.mini_max_ab(move_res.board, depth - 1, a, b, stats));
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
            }, stats.transposition());
        } else if value >= b {
            self.transposition_table.insert(board_hash, Entry {
                depth,
                value,
                tp: EntryType::Upper
            }, stats.transposition());
        } else{
            self.transposition_table.insert(board_hash, Entry {
                depth,
                value,
                tp: EntryType::Exact
            }, stats.transposition());
        }

        return value
    }
}

impl Solver for AlphaBetaTransp {
    fn make_move_impl<B: Board>(&mut self, board: DisplayableBoard<B>, stats: &mut StatsEntry) -> Option<DisplayableBoard<B>> {
        let mut rng = thread_rng();


        let mut best_moves = Vec::new();

        let mut best = 0.0;
        if board.current_player() == White {
            best = f64::NEG_INFINITY;
            for move_res in order_moves(board.all_moves(),&board) {
                let score = self.mini_max_ab(move_res.board, self.search_depth, f64::NEG_INFINITY, f64::INFINITY, stats);
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
            for move_res in order_moves(board.all_moves(),&board) {
                let score = self.mini_max_ab(move_res.board, self.search_depth, f64::NEG_INFINITY, f64::INFINITY, stats);
                if score < best {
                    best = score;
                    best_moves = Vec::new();
                    best_moves.push(move_res.mv)
                } else if score == best {
                    best_moves.push(move_res.mv);
                }
            }
        }

        stats.evaluation(best);

        let board_hash = board.hash();
        self.transposition_table.insert(board_hash, Entry {
            depth: self.search_depth,
            value: best,
            tp: EntryType::Exact
        }, stats.transposition());


        let m = best_moves.into_iter().choose(&mut rng)?;

        Some(board.transition(m))
    }

    fn init_stats(&self, stats_folder: String) -> Stats {
        Stats::new("Minimax with Alpha-Beta pruning using a transposition table", Some(self.search_depth), Some(self.transposition_table.len()), stats_folder, true)
    }

    // fn stats(&self) -> String {
    //     format!("tp hits: {:.3}% explored: {} evaluation: {}, tu: {}, tc: {}", self.table_hits as f64 / self.explored as f64 * 100.0, self.explored, self.last_best, self.transposition_table.used, self.transposition_table.collisions)
    // }
}