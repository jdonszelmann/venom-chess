use crate::game_engine::board::{BasicBoard, Board};
use crate::game_engine::chess_move::Move;
use std::io::Write;
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, Clear};
use crossterm::QueueableCommand;
use crossterm::terminal::ClearType::All;
use crossterm::cursor::MoveTo;
use crossterm::event::{Event, KeyCode, KeyModifiers, MouseEventKind, read, EnableMouseCapture, DisableMouseCapture};
use crate::game_engine::piece::{Piece, queen_of_color, rook_of_color, bishop_of_color, knight_of_color};
use crate::game_engine::color::Color;
use crate::game_engine::chess_move::Extra;
use std::panic::catch_unwind;
use crate::solver::Solver;
use crate::game_engine::board::display::DisplayableBoard;

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


fn mouse_pos_to_coord(x: u16, y: u16) -> (i8, i8) {
    ((x as i8 - 2) / 3, y as i8 - 1)
}

enum ErrorKind {
    Exit,
    UndoMove,
    Redraw,
}


fn get_mouse_input() -> Result<(i8, i8), ErrorKind> {
    let mut stdout = std::io::stdout();

    loop {
        let e = read().map_err(|_| ErrorKind::Exit)?;
        match e {
            Event::Key(k) => {
                if (k.code == KeyCode::Char('c') || k.code == KeyCode::Char('d')) && k.modifiers.contains(KeyModifiers::CONTROL) {
                    return Err(ErrorKind::Exit)
                }

                if k.code == KeyCode::Esc {
                    return Err(ErrorKind::UndoMove);
                }
            },
            Event::Resize(_, _) => {
                return Err(ErrorKind::Redraw)
            }
            Event::Mouse(me) => {
                // println!("{:?}", me.kind);
                if let MouseEventKind::Up(_) = me.kind {
                    let (x, y) = mouse_pos_to_coord(me.column, me.row);

                    if x < 0 || x >= 8 || y < 0 || y >= 8 {
                        continue
                    }

                    return Ok((x, y))
                }
            }
            _ => {}
        }
        stdout.flush().unwrap();
    }
}

fn unix_repl_impl<B: Board, S1: Solver, S2: Solver>(mut board: DisplayableBoard<B>, mut black_solver: Option<S1>, mut white_solver: Option<S2>) -> crossterm::Result<()> {
    let mut stdout = std::io::stdout();
    stdout.flush()?;
    disable_raw_mode()?;
    stdout.flush()?;
    enable_raw_mode()?;

    stdout.queue(EnableMouseCapture)?;

    'outer: loop {
        stdout.queue(Clear(All))?;
        stdout.queue(MoveTo(0, 0))?;

        disable_raw_mode()?;
        stdout.queue(DisableMouseCapture)?;
        println!();

        board.highlight(Vec::new());
        println!("{}", board);
        println!("black solver stats: {}", black_solver.as_ref().map(|i| i.stats()).unwrap_or("".to_string()));
        println!("white solver stats: {}", white_solver.as_ref().map(|i| i.stats()).unwrap_or("".to_string()));
        stdout.flush().expect("couldn't flush stdout");

        if board.current_player() == Color::Black {
            if let Some(ref mut s) = black_solver {
                println!("Solver is making it's move");

                board = match s.make_move(board) {
                    Some(i) => i,
                    None => {
                        println!("Couldn't make move");
                        return Ok(());
                    },
                };
                continue;
            }
        }

        if board.current_player() == Color::White {
            if let Some(ref mut s) = white_solver {
                println!("Solver is making it's move");

                board = match s.make_move(board) {
                    Some(i) => i,
                    None => {
                        println!("Couldn't make move");
                        return Ok(());
                    },
                };
                continue;
            }
        }

        stdout.queue(EnableMouseCapture)?;
        stdout.flush().expect("couldn't flush stdout");
        enable_raw_mode()?;

        let (sx, sy) = loop {
            match get_mouse_input() {
                Ok(a) => break a,
                Err(ErrorKind::Exit) => {
                    return Ok(())
                },
                Err(ErrorKind::UndoMove) => continue,
                Err(ErrorKind::Redraw) => continue 'outer,
            };
        };

        let moves = board.moves((sx, sy));

        stdout.queue(Clear(All))?;
        stdout.queue(MoveTo(0, 0))?;

        disable_raw_mode()?;
        println!();

        if board.piece_at((sx, sy)).color() != board.current_player() {
            println!("that's not your piece!");
            continue;
        }

        board.highlight(moves.iter().map(|i| i.to).collect());
        println!("{}", board);
        stdout.flush().expect("couldn't flush stdout");
        enable_raw_mode()?;

        let (dx, dy) = loop{
            match get_mouse_input() {
                Ok(a) => break a,
                Err(ErrorKind::Exit) => {
                    disable_raw_mode()?;
                    return Ok(())
                },
                Err(ErrorKind::UndoMove) => continue 'outer,
                Err(ErrorKind::Redraw) => continue 'outer,
            };
        };

        let m: Move = ((sx, sy), (dx, dy)).into();

        if let Some(mut i) = moves.iter().find(|&cm| cm.from == m.from && cm.to == m.to).copied() {
            if i.extra.is_promotion() {
                if let Some(e) = do_promotion_input(board.current_player())? {

                    // FIXME: Capturing promotions
                    i.extra = e
                } else {
                    continue
                }
            }

            board = board.transition(i);
        } else {
            println!("Invalid move!");
            continue;
        }
    }
}

fn do_promotion_input(color: Color) -> crossterm::Result<Option<Extra>> {
    let mut stdout = std::io::stdout();

    stdout.queue(Clear(All))?;
    stdout.queue(MoveTo(8, 2))?;


    print!("\x1b[100m{}\x1b[0m", queen_of_color(color));
    print!("\x1b[45m{}\x1b[0m", rook_of_color(color));
    print!("\x1b[100m{}\x1b[0m", bishop_of_color(color));
    print!("\x1b[45m{}\x1b[0m", knight_of_color(color));



    stdout.flush().unwrap();

    let coord = match get_mouse_input() {
        Ok(a) => a,
        Err(ErrorKind::Exit) => return Ok(None),
        Err(ErrorKind::UndoMove) => return Ok(None),
        Err(ErrorKind::Redraw) => return do_promotion_input(color),
    };

    Ok(Some(match coord {
        (2, 1) => Extra::QueenPromotion,
        (3, 1) => Extra::RookPromotion,
        (4, 1) => Extra::BishopPromotion,
        (5, 1) => Extra::KnightPromotion,
        _ => return do_promotion_input(color)
    }))
}

pub fn unix_repl<B: Board, S1: Solver, S2: Solver>(mut board: B, black_solver: Option<S1>, white_solver: Option<S2>) {
    let db = DisplayableBoard::new(board);

    match enable_raw_mode() {
        Ok(_) => (),
        Err(_) => {
            println!("Failed to enable raw mode, using fallback");
            return repl(db, black_solver, white_solver);
        }
    }

    // let _ = catch_unwind(|| {

    match unix_repl_impl(db, black_solver, white_solver) {
        Ok(_) => (),
        Err(e) => println!("{}", e.to_string())
    };

    // });

    let mut stdout = std::io::stdout();
    disable_raw_mode().unwrap();
    stdout.queue(DisableMouseCapture).unwrap();

}

pub fn repl<B: Board, S1: Solver, S2: Solver>(mut board: DisplayableBoard<B>, mut black_solver: Option<S1>, mut white_solver: Option<S2>) {
    let stdin = std::io::stdin();

    loop {
        let mut buf = String::new();
        board.highlight(Vec::new());
        println!("{}", board);
        println!("black solver stats: {}", black_solver.as_ref().map(|i| i.stats()).unwrap_or("".to_string()));
        println!("white solver stats: {}", white_solver.as_ref().map(|i| i.stats()).unwrap_or("".to_string()));
        print!("? ");
        std::io::stdout().flush().expect("couldn't flush stdout");

        if board.current_player() == Color::Black {
            if let Some(ref mut s) = black_solver {
                println!("Solver is making it's move");

                board = match s.make_move(board) {
                    Some(i) => i,
                    None => {
                        println!("Couldn't make move");
                        return ();
                    },
                };
                continue;
            }
        }

        if board.current_player() == Color::White {
            if let Some(ref mut s) = white_solver {
                println!("Solver is making it's move");

                board = match s.make_move(board) {
                    Some(i) => i,
                    None => {
                        println!("Couldn't make move");
                        return ();
                    },
                };
                continue;
            }
        }

        stdin.read_line(&mut buf).expect("couldn't read line from stdin");

        let (sx, sy) = if let Some(a) = parse_input(&buf) {
            a
        } else {
            println!("couldn't parse input. Please specify a location by separating two integers by whitespace");
            continue;
        };

        let moves = board.moves((sx, sy));

        if board.piece_at((sx, sy)).color() != board.current_player() {
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

        if let Some(mut i) = moves.iter().find(|&cm| cm.from == m.from && cm.to == m.to).copied() {
            if i.extra.is_promotion() {
                if let Some(e) = do_promotion_input_fallback(board.current_player()).unwrap() {

                    // FIXME: Capturing promotions
                    i.extra = e
                } else {
                    continue
                }
            }

            board = board.transition(i);
        } else {
            println!("Invalid move!");
            continue;
        }
    }
}

fn do_promotion_input_fallback(color: Color) -> crossterm::Result<Option<Extra>> {
    println!("1: Queen");
    println!("2: Rook");
    println!("3: Bishop");
    println!("4: Knight");

    let mut stdin = std::io::stdin();

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