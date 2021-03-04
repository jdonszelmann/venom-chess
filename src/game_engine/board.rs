use crate::game_engine::piece::Piece;
use crate::game_engine::piece::Piece::*;
use crate::game_engine::chessmove::Move;
use crate::game_engine::chessMove::{Move, Location};
use std::fmt;
use crate::game_engine::piece::Color::White;


#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct Board {
    board: [[Piece; 8]; 8]
}


impl Board {
    const GAME_START: Board = Board { board: [
        [BlackRook, BlackKnight, BlackBishop, BlackQueen, BlackKing, BlackBishop, BlackKnight, BlackRook],
        [BlackPawn, BlackPawn, BlackPawn, BlackPawn, BlackPawn, BlackPawn, BlackPawn, BlackPawn],
        [Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
        [Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
        [Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
        [Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
        [WhitePawn, WhitePawn, WhitePawn, WhitePawn, WhitePawn, WhitePawn, WhitePawn, WhitePawn],
        [WhiteRook, WhiteKnight, WhiteBishop, WhiteQueen, WhiteKing, WhiteBishop, WhiteKnight, WhiteRook],
    ]};

    pub fn new() -> Self {
        Self {
            board: [[Empty; 8]; 8]
        }
    }

    pub fn possible_moves(&self) -> Vec<Move> {

    }

    pub fn transition(&self, m: Move) -> Self {

    }

    pub fn piece_at(&self, l: Location) -> Piece {
        self.board[l.x][l.y]
    }


}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for x in 0..8 {
            for y in 0..8 {
                if (x + y) % 2 {
                    write!(f, "\033[100m")?;
                } else {
                    write!(f, "\033[45m")?;
                }

                write!(f, "{}", self.piece_at((x, y).into()))?;
                write!(f, "\033[0m")?;

            }
            writeln!(f);
        }

        Ok(())
    }
}