use crate::solver::Solver;
use crate::stats::{StatsEntry, Stats};
use crate::game_engine::board::Board;
use std::io::Write;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear};
use crossterm::QueueableCommand;
use crossterm::event::{EnableMouseCapture, DisableMouseCapture, Event, KeyCode, KeyModifiers, MouseEventKind, read};
use crossterm::cursor::MoveTo;
use crossterm::terminal::ClearType::All;
use crate::game_engine::chess_move::{Move, Extra};
use crate::game_engine::board::display::DisplayableBoard;
use crate::game_engine::color::Color;
use crate::game_engine::piece::{queen_of_color, rook_of_color, bishop_of_color, knight_of_color};

pub struct AnsiPlayer {

}

impl AnsiPlayer {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl Solver for AnsiPlayer {
    const PRINT_OWN_BOARD: bool = true;

    fn make_move_impl<B: Board>(&mut self, board: DisplayableBoard<B>, stats: &mut StatsEntry) -> Option<DisplayableBoard<B>> {
        let mv = match make_move_input(board.clone(), stats) {
            Ok(i) => i,
            Err(_) => {
                disable_raw_mode().unwrap();
                std::io::stdout().queue(DisableMouseCapture).unwrap();
                return None
            }
        };

        Some(board.transition(mv))
    }

    fn init_stats(&self, stats_folder: String) -> Stats {
        Stats::new("Human player (Ansi input)", None, None, stats_folder, false)
    }
}

fn make_move_input<B: Board>(board: DisplayableBoard<B>, _stats: &mut StatsEntry) -> crossterm::Result<Move> {
    let mut b = board;

    'outer: loop {
        let mut stdout = std::io::stdout();
        stdout.flush()?;
        disable_raw_mode()?;
        stdout.flush()?;
        enable_raw_mode()?;

        stdout.queue(Clear(All))?;
        stdout.queue(MoveTo(0, 0))?;

        disable_raw_mode()?;
        println!();

        b.highlight(Vec::new());
        println!("{}", b);

        stdout.queue(EnableMouseCapture)?;
        stdout.flush()?;
        enable_raw_mode()?;

        let (sx, sy) = loop {
            match get_mouse_input() {
                Ok(a) => break a,
                Err(ErrorKind::Exit) => {
                    return Err(Into::<std::io::Error>::into(std::io::ErrorKind::Other).into());
                },
                Err(ErrorKind::UndoMove) => continue,
                Err(ErrorKind::Redraw) => continue 'outer,
            };
        };

        let moves = b.moves((sx, sy));

        stdout.queue(Clear(All))?;
        stdout.queue(MoveTo(0, 0))?;

        disable_raw_mode()?;
        println!();

        if b.piece_at((sx, sy)).color() != b.current_player() {
            println!("that's not your piece!");
            continue;
        }

        b.highlight(moves.iter().map(|i| i.to).collect());
        println!("{}", b);
        stdout.flush().expect("couldn't flush stdout");
        enable_raw_mode()?;

        let (dx, dy) = loop{
            match get_mouse_input() {
                Ok(a) => break a,
                Err(ErrorKind::Exit) => {
                    disable_raw_mode()?;
                    return Err(Into::<std::io::Error>::into(std::io::ErrorKind::Other).into());
                },
                Err(ErrorKind::UndoMove) => continue 'outer,
                Err(ErrorKind::Redraw) => continue 'outer,
            };
        };

        let m: Move = ((sx, sy), (dx, dy)).into();

        if let Some(mut i) = moves.iter().find(|&cm| cm.from == m.from && cm.to == m.to).copied() {
            if i.extra.is_promotion() {
                if let Some(e) = do_promotion_input(b.current_player())? {

                    // FIXME: Capturing promotions
                    i.extra = e
                } else {
                    continue
                }
            }

            disable_raw_mode()?;
            stdout.queue(DisableMouseCapture)?;
            stdout.queue(Clear(All))?;
            stdout.queue(MoveTo(0, 0))?;
            println!();

            return Ok(i);
        } else {
            println!("Invalid move!");
        }
    }
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
        }
        stdout.flush().unwrap();
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