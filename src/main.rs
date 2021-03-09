use crate::game_engine::board::zobrist::ZobristBoard;
use crate::game_engine::board::pst::PSTBoard;
use crate::game_engine::board::BasicBoard;
use crate::solver::player::Player;
use crate::solver::quiescence::Quiescence;
use crate::runner::Runner;

mod game_engine;
mod solver;
mod runner;
mod transposition_table;
mod stats;


fn main() {
    let b = ZobristBoard::new(PSTBoard::new(BasicBoard::DEFAULT_BOARD));

    let p1 = Player::new();
    let p2 = Quiescence::new(5);

    let mut r = Runner::new(p1, p2);
    r.run(b);

    // let mut b = BasicBoard::new();
    // *b.piece_at_mut((6, 1)) = Piece::WhitePawn;
    // *b.piece_at_mut((0, 7)) = Piece::WhiteKing;
    // *b.piece_at_mut((7, 7)) = Piece::BlackKing;
    // b.current = Color::White;

    // let white_solver = AlphaBetaTransp::new(5, 1024 * 1024 * 8);
    // let black_solver = Quiescence::new(5);

    // unix_repl::<_, _, _>(b, Some(black_solver), Some(white_solver));
    // unix_repl::<_, AlphaBetaTransp, _>(b, None, Some(black_solver));

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
