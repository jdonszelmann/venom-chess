use crate::game_engine::board::{BasicBoard, Board};
use crate::game_engine::chess_move::Move;
use std::io::Write;
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, Clear};
use crossterm::QueueableCommand;
use crossterm::terminal::ClearType::All;
use crossterm::cursor::MoveTo;
use crossterm::event::{Event, KeyCode, KeyModifiers, MouseEventKind, read, EnableMouseCapture, DisableMouseCapture};

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
    ((x - 2) as i8 / 3, y as i8 - 1)
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

fn unix_repl_impl(mut board: BasicBoard) -> crossterm::Result<()> {
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
        println!();

        board.highlight(Vec::new());
        println!("{}", board);
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

        if board.piece_at((sx, sy)).color() != board.current {
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

        if !moves.contains(&m) {
            println!("Invalid move!");
            continue;
        }

        board = board.transition(m);
    }
}

pub fn unix_repl(mut board: BasicBoard) {
    match enable_raw_mode() {
        Ok(_) => (),
        Err(_) => {
            println!("Failed to enable raw mode, using fallback");
            return repl(board);
        }
    }

    match unix_repl_impl(board) {
        Ok(_) => (),
        Err(e) => println!("{}", e.to_string())
    };

    let mut stdout = std::io::stdout();
    disable_raw_mode().unwrap();
    stdout.queue(DisableMouseCapture).unwrap();
}

pub fn repl(mut board: BasicBoard) {
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