use crate::game_engine::piece::{Piece, Color};
use crate::game_engine::piece::Piece::*;
use crate::game_engine::chessMove::{Move, Location};
use std::fmt;
use crate::game_engine::piece::Color::White;
use std::io::SeekFrom::Current;


#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct Board {
    board: [[Piece; 8]; 8],

    current: Color,
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
    };

    pub fn new() -> Self {
        Self {
            board: [[Empty; 8]; 8],
            current: White,
        }
    }

    pub fn possible_moves(&self) -> Vec<Move> {
        todo!()
    }

    pub fn transition(&self, m: Move) -> Self {
        let mut new_board = self.clone();
        let movable = self.piece_at(m.from);

        *new_board.piece_at_mut(m.to) = movable;
        *new_board.piece_at_mut(m.from)= Piece::Empty;

        new_board.current = self.current.other();

        new_board
    }

    pub fn piece_at(&self, l: Location) -> Piece {
        self.board[l.y as usize][l.x as usize]
    }

    pub fn piece_at_mut(&mut self, l: Location) -> &mut Piece {
        &mut self.board[l.y as usize][l.x as usize]
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

                write!(f, "{}", self.piece_at((x, y).into()))?;
                write!(f, "\x1b[0m")?;

            }
            writeln!(f);
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