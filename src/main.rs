use crate::game_engine::board::Board;
use crate::ui::unix_repl;

mod game_engine;
mod ai;
mod ui;




fn main() {
    let b = Board::DEFAULT_BOARD;
    // let mut b = Board::new();
    // *b.piece_at_mut((6, 7).into()) = Piece::BlackKing;
    // *b.piece_at_mut((7, 5).into()) = Piece::WhiteKing;
    // *b.piece_at_mut((5, 7).into()) = Piece::BlackPawn;
    // b.current = Color::Black;

    unix_repl(b);

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
