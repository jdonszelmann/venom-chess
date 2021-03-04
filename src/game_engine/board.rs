use crate::game_engine::piece::Piece;
use crate::game_engine::piece::Piece::Empty;
use crate::game_engine::chessmove::Move;
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

    }

    pub fn transition(&self, m: Move) -> Self {

    }

    pub fn piece_at(&self, l: Location) -> Piece {
        self.board[l.x][l.y]
    }


}