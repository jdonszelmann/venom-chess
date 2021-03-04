
mod game_engine;
mod ai;

use game_engine::board::Board;
use std::io::Write;
use crate::game_engine::chessMove::Move;

fn parse_input(input: &str) -> Option<(u8, u8)> {
    let mut i = input.trim().split_ascii_whitespace();
    let first = i.next()?;
    let second = i.next()?;

    let first_int: u8 = first.parse().ok()?;
    let second_int: u8 = second.parse().ok()?;

    if first_int < 0 || first_int >= 8 {
        return None
    }

    if second_int < 0 || second_int >= 8 {
        return None
    }


    Some((first_int, second_int))
}

fn repl(mut board: Board) {
    let mut stdin = std::io::stdin();

    loop {
        let mut buf = String::new();
        println!("{}", board);
        print!("? ");
        std::io::stdout().flush();

        stdin.read_line(&mut buf).expect("couldn't read line from stdin");

        let (sx, sy) = if let Some(a) = parse_input(&buf) {
            a
        } else {
            println!("couldn't parse input. Please specify a location by separating two integers by whitespace");
            continue;
        };

        print!("? ");
        std::io::stdout().flush();

        let mut buf = String::new();
        stdin.read_line(&mut buf).expect("couldn't read line from stdin");

        let (dx, dy) = if let Some(a) = parse_input(&buf) {
            a
        } else {
            println!("couldn't parse input. Please specify a location by separating two integers by whitespace");
            continue;
        };

        let m: Move = ((sx, sy), (dx, dy)).into();

        let moves = board.piece_at(m.from).moves(m.from, &board);
        if !moves.contains(&m) {
            println!("Invalid move!");
            continue;
        }

        board = board.transition(m);
    }
}

fn main() {
    let b = Board::DEFAULT_BOARD;

    repl(b)
}
