use crate::game_engine::board::Board;
use termion::raw::{IntoRawMode, RawTerminal};
use std::io::{Write, Stdout};
use crate::game_engine::chess_move::Move;
use termion::input::{MouseTerminal, TermRead};
use termion::event::{Event, MouseEvent, Key};
use termion::{cursor, terminal_size};
use std::thread;
use std::time::Duration;


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
    ((x - 3) as i8 / 3, y as i8 - 2)
}

enum ErrorKind {
    Exit,
    UndoMove,
    Redraw,
}


fn get_mouse_input(stdout: &mut MouseTerminal<RawTerminal<Stdout>>) -> Result<(i8, i8), ErrorKind> {
    let size = terminal_size().unwrap();

    for c in std::io::stdin().events() {
        let new_size = terminal_size().unwrap();
        if new_size != size {
            return Err(ErrorKind::Redraw)
        }


        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Ctrl('c')) => return Err(ErrorKind::Exit),
            Event::Key(Key::Ctrl('d')) => return Err(ErrorKind::Exit),
            Event::Key(Key::Esc) => return Err(ErrorKind::UndoMove),
            Event::Mouse(me) => {
                match me {
                    MouseEvent::Release(a, b) |
                    MouseEvent::Hold(a, b) => {
                        let (x, y) = mouse_pos_to_coord(a, b);

                        if x < 0 || x >= 8 || y < 0 || y >= 8 {
                            continue
                        }

                        return Ok((x, y))
                    }
                    _ => (),
                }
            }
            _ => {}
        }
        stdout.flush().unwrap();
    }

    return Err(ErrorKind::Exit)
}

pub fn unix_repl(mut board: Board) {
    let mut stdout = MouseTerminal::from(match std::io::stdout().into_raw_mode() {
        Ok(i) => i,
        Err(_) => {
            println!("failed to switch to raw ANSI user input mode");
            return repl(board)
        },
    });


    'outer: loop {
        print!("{}", termion::clear::All);
        print!("{}", cursor::Goto(1, 1));

        stdout.suspend_raw_mode().unwrap();
        println!();

        board.highlight(Vec::new());
        println!("{}", board);
        stdout.flush().expect("couldn't flush stdout");
        stdout.activate_raw_mode().unwrap();

        let (sx, sy) = loop {
            match get_mouse_input(&mut stdout) {
                Ok(a) => break a,
                Err(ErrorKind::Exit) => return,
                Err(ErrorKind::UndoMove) => continue,
                Err(ErrorKind::Redraw) => continue 'outer,
            };
        };

        let moves = board.moves((sx, sy));

        print!("{}", termion::clear::All);
        print!("{}", cursor::Goto(1, 1));

        stdout.suspend_raw_mode().unwrap();
        println!();

        if board.piece_at((sx, sy)).color() != board.current {
            println!("that's not your piece!");
            continue;
        }

        board.highlight(moves.iter().map(|i| i.to).collect());
        println!("{}", board);
        stdout.flush().expect("couldn't flush stdout");
        stdout.activate_raw_mode().unwrap();

        let (dx, dy) = loop{
            match get_mouse_input(&mut stdout) {
                Ok(a) => break a,
                Err(ErrorKind::Exit) => return,
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


pub fn repl(mut board: Board) {
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