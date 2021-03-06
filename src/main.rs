use crate::game_engine::board::BasicBoard;
use crate::ui::unix_repl;
use crate::game_engine::piece::Piece;
use crate::game_engine::color::Color;
use crate::solver::minimax::Minimax;
use std::thread;
use std::time::Duration;
use crate::solver::alpha_beta::AlphaBeta;
use crate::solver::Solver;
use crate::game_engine::board::pst::PSTBoard;

mod game_engine;
mod solver;
mod ui;




fn main() {
    let mut b = PSTBoard::new(BasicBoard::DEFAULT_BOARD);
    // let mut b = BasicBoard::new();
    // *b.piece_at_mut((6, 1)) = Piece::WhitePawn;
    // *b.piece_at_mut((0, 7)) = Piece::WhiteKing;
    // *b.piece_at_mut((7, 7)) = Piece::BlackKing;
    // b.current = Color::White;

    let solver = AlphaBeta::new(4);

    unix_repl::<_, AlphaBeta>(b, Some(solver));

    // let rp = AlphaBeta::new();
    // loop {
    //     // thread::sleep(Duration::from_millis(1000));
    //
    //     if let Some(i) = rp.make_move(b) {
    //         b = i;
    //     } else {
    //         println!("No moves left");
    //         break
    //     }
    //     println!("{}", b);
    // }
}
