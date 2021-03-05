use crate::game_engine::board::{BasicBoard, Board};
use crate::ui::unix_repl;
use crate::game_engine::piece::Piece;
use crate::game_engine::color::Color;

mod game_engine;
mod ai;
mod ui;




fn main() {
    // let b = BasicBoard::DEFAULT_BOARD;
    let mut b = BasicBoard::new();
    *b.piece_at_mut((6, 1)) = Piece::WhitePawn;
    *b.piece_at_mut((0, 7)) = Piece::WhiteKing;
    *b.piece_at_mut((7, 7)) = Piece::BlackKing;
    b.current = Color::White;

    unix_repl::<Minimax>(b, None);

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
