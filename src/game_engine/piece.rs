use crate::game_engine::board::Board;
use crate::game_engine::chessMove::{Location, Move};

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum Piece {
    Empty,

    Pawn,

    Bishop,
    Knight,
    Rook,

    King,
    Queen,
}


impl Piece {
    pub fn is_empty(&self) -> bool {
        self == Piece::Empty
    }

    pub fn moves(&self, location: Location, board: Board) -> Vec<Move> {
        match self {
            Self::Empty => Vec::new(),
            Piece::Pawn => {}
            Piece::Bishop => {}
            Piece::Knight => {}
            Piece::Rook => {}
            Piece::King => {}
            Piece::Queen => {}
        }
    }
}

fn pawn_moves(location: Location, board: Board) -> Vec<Move> {

}

fn bishop_moves(location: Location, board: Board) -> Vec<Move> {

}

fn knight_moves(location: Location, board: Board) -> Vec<Move> {

}

fn rook_moves(location: Location, board: Board) -> Vec<Move> {

}

fn king_moves(location: Location, board: Board) -> Vec<Move> {

}

fn queen_moves(location: Location, board: Board) -> Vec<Move> {

}