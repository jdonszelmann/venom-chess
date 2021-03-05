use crate::game_engine::piece::{Piece, Color};
use crate::game_engine::piece::Piece::*;
use crate::game_engine::chess_move::{Move, Location};
use std::fmt;
use crate::game_engine::piece::Color::{White, Black};
use std::io::SeekFrom::Current;


#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct Board {
    pub board: [[Piece; 8]; 8],

    pub current: Color,

    pub highlighted: Vec<Location>
}

impl Board {
    pub const DEFAULT_BOARD: Board = Board {
        board: [
            [BlackRook, BlackKnight, BlackBishop, BlackQueen, BlackKing, BlackBishop, BlackKnight, BlackRook],
            [BlackPawn, BlackPawn, BlackPawn, BlackPawn, BlackPawn, BlackPawn, BlackPawn, BlackPawn],
            [Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
            [Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
            [Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
            [Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
            [WhitePawn, WhitePawn, WhitePawn, WhitePawn, WhitePawn, WhitePawn, WhitePawn, WhitePawn],
            [WhiteRook, WhiteKnight, WhiteBishop, WhiteQueen, WhiteKing, WhiteBishop, WhiteKnight, WhiteRook],
        ],

        current: White,

        highlighted: Vec::new(),
    };

    pub fn new() -> Self {
        Self {
            board: [[Empty; 8]; 8],
            current: White,
            highlighted: Vec::new(),
        }
    }

    #[inline]
    pub fn get_pieces<'a>(&'a self) -> impl Iterator<Item = (Piece, Location)> + 'a {
        (0..8).map(move |i| (0..8).map(move |j| {
            let l = (i, j).into();
            (self.piece_at(l), l)
        }))
            .flatten()
            .filter(|(i, _)| !i.is_empty())
            .filter(move |(i, _)| i.color() == self.current)
    }

    #[inline]
    pub fn possible_moves<'a>(&'a self) -> impl Iterator<Item = Move> + 'a {
        self.get_pieces()
            .map(move |(p, l)| self.moves(l))
            .flatten()
    }

    pub fn transition(&self, m: Move) -> Self {
        let mut new_board = self.clone();
        let movable = self.piece_at(m.from);

        *new_board.piece_at_mut(m.to) = movable;
        *new_board.piece_at_mut(m.from)= Piece::Empty;

        new_board.current = self.current.other();

        new_board
    }

    #[inline]
    pub(crate) fn moves(&self, l: impl Into<Location> + Copy) -> Vec<Move> {
        self.piece_at(l)
            .moves(l, self)
            .into_iter()
            .filter(move |&i| !self.transition(i).king_check(self.current))
            .collect()
    }

    pub fn piece_at(&self, l: impl Into<Location>) -> Piece {
        let l = l.into();
        self.board[l.y as usize][l.x as usize]
    }

    pub fn piece_at_mut(&mut self, l: Location) -> &mut Piece {
        &mut self.board[l.y as usize][l.x as usize]
    }

    pub fn king_location(&self, color: Color) -> Option<Location> {
        if color == Color::White {
            for y in 0..8 {
                for x in 0..8 {
                    let other = self.piece_at((x, y));
                    if other == Piece::WhiteKing{
                        return Some((x,y).into());
                    }
                }
            }
        } else {
            for y in 0..8 {
                for x in 0..8 {
                    let other = self.piece_at((x, y));
                    if other == Piece::BlackKing{
                        return Some((x,y).into());
                    }
                }
            }
        }
        None
    }



    pub fn highlight(&mut self, locations: Vec<Location>) {
        self.highlighted = locations;
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..8 {
            write!(f, "{} ", y)?;

            for x in 0..8 {

                if (x + y) % 2 == 0{
                    write!(f, "\x1b[100m")?;
                } else {
                    write!(f, "\x1b[45m")?;
                }

                if self.highlighted.contains(&(x, y).into()) {
                    write!(f, "\x1b[103m")?;
                }

                write!(f, "{}", self.piece_at((x, y)))?;
                write!(f, "\x1b[0m")?;

            }
            writeln!(f)?;
        }

        writeln!(f, "   0  1  2  3  4  5  6  7 ")?;
        if self.current == Color::White {
            writeln!(f, "current player: White")?;
        } else {
            writeln!(f, "current player: Black")?;
        }

        Ok(())
    }
}