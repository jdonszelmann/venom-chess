use crate::game_engine::piece::Piece;
use crate::game_engine::piece::Piece::Empty;
use crate::game_engine::chessMove::{Move, Location};


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