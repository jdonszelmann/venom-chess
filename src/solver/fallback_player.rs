use crate::solver::Solver;
use crate::stats::{StatsEntry, Stats};
use crate::game_engine::board::Board;
use crate::game_engine::chess_move::{Extra, Move};
use crate::game_engine::board::display::DisplayableBoard;
use std::io::Write;
use crate::game_engine::color::Color;

pub struct FallbackPlayer {

}

impl FallbackPlayer {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl Solver for FallbackPlayer {
    const PRINT_OWN_BOARD: bool = true;

    fn make_move_impl<B: Board>(&mut self, board: DisplayableBoard<B>, stats: &mut StatsEntry) -> Option<DisplayableBoard<B>> {
        let mv = make_move_input(board.clone(), stats)?;

        Some(board.transition(mv))
    }

    fn init_stats(&self, stats_folder: String) -> Stats {
        Stats::new("Human player (Fallback input)", None, None, stats_folder, false)
    }
}

pub fn make_move_input<B: Board>(board: DisplayableBoard<B>, _stats: &mut StatsEntry) -> Option<Move> {
    let stdin = std::io::stdin();
    let mut b = board;

    loop {
        let mut buf = String::new();
        b.highlight(Vec::new());
        println!("{}", b);
        print!("? ");
        std::io::stdout().flush().expect("couldn't flush stdout");

        stdin.read_line(&mut buf).expect("couldn't read line from stdin");

        let (sx, sy) = if let Some(a) = parse_input(&buf) {
            a
        } else {
            println!("couldn't parse input. Please specify a location by separating two integers by whitespace");
            continue;
        };

        let moves = b.moves((sx, sy));

        if b.piece_at((sx, sy)).color() != b.current_player() {
            println!("that's not your piece!");
            continue;
        }

        b.highlight(moves.iter().map(|i| i.to).collect());
        println!("{}", b);
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

        if let Some(mut i) = moves.iter().find(|&cm| cm.from == m.from && cm.to == m.to).copied() {
            if i.extra.is_promotion() {
                if let Some(e) = do_promotion_input_fallback(b.current_player()).unwrap() {

                    // FIXME: Capturing promotions
                    i.extra = e
                } else {
                    continue
                }
            }

            return Some(i)
        } else {
            println!("Invalid move!");
        }
    }
}

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

fn do_promotion_input_fallback(color: Color) -> crossterm::Result<Option<Extra>> {
    println!("1: Queen");
    println!("2: Rook");
    println!("3: Bishop");
    println!("4: Knight");

    let stdin = std::io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).expect("couldn't read line from stdin");

    let value: u64 = match buf.trim().parse() {
        Ok(i) => i,
        Err(_) => {
            println!("Couldn't parse input as number");
            return do_promotion_input_fallback(color);
        },
    };

    Ok(Some(match value {
        1 => Extra::QueenPromotion,
        2 => Extra::RookPromotion,
        3 => Extra::BishopPromotion,
        4 => Extra::KnightPromotion,
        _ => return do_promotion_input_fallback(color)
    }))
}