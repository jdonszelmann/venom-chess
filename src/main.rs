
mod game_engine;
mod ai;

use game_engine::board::Board;
use std::io::Write;
use crate::game_engine::chess_move::Move;
use crate::ai::random_play::RandomPlay;
use std::time::Duration;
use std::thread;
use crate::game_engine::piece::{Piece, Color};

fn parse_input(input: &str) -> Option<(i8, i8)> {
    let mut i = input.trim().split_ascii_whitespace();
    let first = i.next()?;
    let second = i.next()?;

    let first_int: i8 = first.parse().ok()?;
    let second_int: i8 = second.parse().ok()?;

    if first_int < 0 || first_int >= 8 {
        return None
    }

    if second_int < 0 || second_int >= 8 {
        return None
    }


    Some((first_int, second_int))
}

fn repl(mut board: Board) {
    let stdin = std::io::stdin();

    loop {
        let mut buf = String::new();
        board.highlight(Vec::new());
        println!("{}", board);
        print!("? ");
        std::io::stdout().flush().expect("couldn't flush stdout");

        stdin.read_line(&mut buf).expect("couldn't read line from stdin");

        let (sx, sy) = if let Some(a) = parse_input(&buf) {
            a
        } else {
            println!("couldn't parse input. Please specify a location by separating two integers by whitespace");
            continue;
        };

        let moves = board.moves((sx, sy));

        if board.piece_at((sx, sy)).color() != board.current {
            println!("that's not your piece!");
            continue;
        }

        board.highlight(moves.iter().map(|i| i.to).collect());
        println!("{}", board);

        print!("? ");
        std::io::stdout().flush().expect("couldn't flush stdout");

        let mut buf = String::new();
        stdin.read_line(&mut buf).expect("couldn't read line from stdin");

        let (dx, dy) = if let Some(a) = parse_input(&buf) {
            a
        } else {
            println!("couldn't parse input. Please specify a location by separating two integers by whitespace");
            continue;
        };

        let m: Move = ((sx, sy), (dx, dy)).into();

        if !moves.contains(&m) {
            println!("Invalid move!");
            continue;
        }

        board = board.transition(m);
    }
}

fn main() {
    // let mut b = Board::DEFAULT_BOARD;
    let mut b = Board::new();
    *b.piece_at_mut((7, 7).into()) = Piece::BlackKing;
    // *b.piece_at_mut((6, 6).into()) = Piece::WhiteRook;
    *b.piece_at_mut((5, 4).into()) = Piece::WhiteKing;
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
