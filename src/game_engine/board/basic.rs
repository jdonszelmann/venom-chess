
use crate::game_engine::chess_move::{Location, Move};
use crate::game_engine::piece::Piece;
use crate::game_engine::color::Color::*;
use crate::game_engine::color::Color;
use crate::game_engine::piece::Piece::*;
use std::fmt;
use crate::game_engine::board::Board;
use crate::game_engine::king_check::king_check;
use std::iter::{Filter, Flatten, Map};
use std::ops::Range;
use crate::game_engine::piece_moves::{pawn_moves_black, pawn_moves_white, bishop_moves, knight_moves, rook_moves, king_moves, queen_moves};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct BasicBoard {
    pub board: [[Piece; 8]; 8],

    pub current: Color,

    pub highlighted: Vec<Location>
}

impl BasicBoard {
    pub const DEFAULT_BOARD: BasicBoard = BasicBoard {
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


    pub fn highlight(&mut self, locations: Vec<Location>) {
        self.highlighted = locations;
    }
}

impl Board for BasicBoard {

    fn moves(&self, location: impl Into<Location>) -> Vec<Move> {
        let location = location.into();
        match self.piece_at(location) {
            Piece::Empty => Vec::new(),

            Piece::BlackPawn => pawn_moves_black(location, self),
            Piece::WhitePawn => pawn_moves_white(location, self),

            Piece::BlackBishop => bishop_moves(location, self),
            Piece::WhiteBishop => bishop_moves(location, self),

            Piece::BlackKnight => knight_moves(location, self),
            Piece::WhiteKnight => knight_moves(location, self),

            Piece::BlackRook => rook_moves(location, self),
            Piece::WhiteRook => rook_moves(location, self),

            Piece::BlackKing => king_moves(location, self),
            Piece::WhiteKing => king_moves(location, self),

            Piece::BlackQueen => queen_moves(location, self),
            Piece::WhiteQueen => queen_moves(location, self),
        }.into_iter()
            .filter(move |&i| !king_check(self.transition(i), self.current))
            .collect()
    }

    fn all_moves(&self) -> Vec<Move> {
        self.all_pieces()
            .into_iter()
            .map(move |(_, l)| self.moves(l))
            .flatten()
            .collect()
    }

    fn transition(&self, m: Move) -> Self {
        let mut new_board = self.clone();
        let movable = self.piece_at(m.from);

        *new_board.piece_at_mut(m.to) = movable;
        *new_board.piece_at_mut(m.from)= Piece::Empty;

        new_board.current = self.current.other();

        new_board
    }

    fn all_pieces(&self) -> Vec<(Piece, Location)> {
        (0..8).map(move |i| (0..8).map(move |j| {
            let l = (i, j).into();
            (self.piece_at(l), l)
        }))
            .flatten()
            .filter(|(i, _)| !i.is_empty())
            .filter(move |(i, _)| i.color() == self.current)
            .collect()
    }

    fn is_terminal(&self) -> Option<Color> {
        unimplemented!()
    }

    fn piece_at(&self, l: impl Into<Location>) -> Piece {
        let l = l.into();
        self.board[l.y as usize][l.x as usize]
    }

    fn piece_at_mut(&mut self, l: impl Into<Location>) -> &mut Piece {
        let l = l.into();
        &mut self.board[l.y as usize][l.x as usize]
    }
}

impl fmt::Display for BasicBoard {
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
