use crate::game_engine::board::{BasicBoard, Board};
use crate::ui::unix_repl;
use crate::game_engine::piece::Piece;
use crate::game_engine::color::Color;
use crate::solver::minimax::Minimax;
use std::thread;
use std::time::Duration;

mod game_engine;
mod solver;
mod ui;




fn main() {
    // let mut b = BasicBoard::DEFAULT_BOARD;
    let mut b = BasicBoard::new();
    *b.piece_at_mut((6, 0)) = Piece::WhiteQueen;
    *b.piece_at_mut((7, 0)) = Piece::WhiteQueen;

    *b.piece_at_mut((3, 7)) = Piece::WhiteKing;
    *b.piece_at_mut((4, 1)) = Piece::BlackKing;
    b.current = Color::Black;

    // let solver = Minimax::new();

    unix_repl::<Minimax>(b, None);

    // let rp = Minimax::new();
    // loop {
    //     thread::sleep(Duration::from_millis(1000));
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
