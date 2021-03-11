use crate::game_engine::board::Board;
use crate::game_engine::color::Color;
use crate::solver::Solver;
use crate::stats::Stats;
use crate::game_engine::color::Color::White;
use crate::game_engine::board::display::DisplayableBoard;

pub struct Runner<S1, S2> {
    black_solver: S1,
    white_solver: S2,

    black_stats: Stats,
    white_stats: Stats,
}

impl<S1: Solver, S2: Solver> Runner<S1, S2> {
    pub fn new(black_solver: S1, white_solver: S2, stats_folder: String) -> Self {
        Self {
            black_stats: black_solver.init_stats(stats_folder.clone()),
            white_stats: white_solver.init_stats(stats_folder),

            black_solver,
            white_solver,
        }
    }

    pub fn run<B: Board>(&mut self, board: B) -> Color {
        let mut db = DisplayableBoard::new(board);
        let mut no_move_counter = 0;

        loop {
            if let Some(i) = db.is_terminal() {
                println!("{}", db);
                println!("{:?}", db.get_clock());

                return i;
            }

            if no_move_counter > 0 {
                unreachable!();
            }

            if db.current_player() == Color::Black {
                if !S1::PRINT_OWN_BOARD {
                    println!("{}", db);
                    println!("{:?}", db.get_clock());
                    // println!("black stats: {:?}", self.black_stats.last_entry());
                    // println!("white stats: {:?}", self.white_stats.last_entry());
                }

                db = match self.black_solver.make_move(db.clone(), self.black_stats.clone()) {
                    Some(i) => i,
                    None => {
                        println!("black couldn't make a move");
                        no_move_counter += 1;
                        continue;
                    }
                };
            } else {
                if !S2::PRINT_OWN_BOARD {
                    println!("{}", db);
                    println!("{:?}", db.get_clock());
                    // println!("black stats: {:?}", self.black_stats.last_entry());
                    // println!("white stats: {:?}", self.white_stats.last_entry());
                }
                db = match self.white_solver.make_move(db.clone(), self.white_stats.clone()) {
                    Some(i) => i,
                    None => {
                        println!("white couldn't make a move");
                        no_move_counter += 1;

                        continue;
                    }
                };
            }
        }
    }
}