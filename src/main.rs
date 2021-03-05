use crate::game_engine::board::{BasicBoard, Board};
use crate::ui::repl;
use crate::game_engine::piece::Piece;
use crate::game_engine::color::Color;

mod game_engine;
mod ai;
mod ui;




fn main() {
    // let b = Board::DEFAULT_BOARD;
    let mut b = BasicBoard::new();
    *b.piece_at_mut((6, 6)) = Piece::BlackKing;
    *b.piece_at_mut((7, 5)) = Piece::WhiteKing;
    *b.piece_at_mut((5, 7)) = Piece::BlackPawn;
    b.current = Color::Black;

    repl(b);

    // let rp = RandomPlay::new();
    // loop {
    //     thread::sleep(Duration::from_millis(400));
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
