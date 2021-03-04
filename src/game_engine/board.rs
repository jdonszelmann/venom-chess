use crate::game_engine::piece::Piece;
use crate::game_engine::piece::Piece::Empty;
use crate::game_engine::chessMove::{Move, Location};
use std::fmt;


#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct Board {
    board: [[Piece; 8]; 8]
}


impl Board {
    pub fn new() -> Self {
        Self {
            board: [[Empty; 8]; 8]
        }
    }

    pub fn possible_moves(&self) -> Vec<Move> {
        todo!()
    }

    pub fn transition(&self, m: Move) -> Self {
        todo!()
    }

    pub fn piece_at(&self, l: Location) -> Piece {
        self.board[l.x as usize][l.y as usize]
    }


}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for x in 0..8 {
            for y in 0..8 {
                if (x + y) % 2 == 0{
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