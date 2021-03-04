use crate::game_engine::board::Board;
use crate::game_engine::chessMove::{Location, Move};
use crate::game_engine::piece::Color::{White, Empty, Black};

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum Piece {
    Empty,

    BlackPawn,
    WhitePawn,


    BlackBishop,
    WhiteBishop,

    BlackKnight,
    WhiteKnight,

    BlackRook,
    WhiteRook,

    BlackKing,
    WhiteKing,

    BlackQueen,
    WhiteQueen,
}


impl Piece {
    pub fn is_empty(&self) -> bool {
        self == Piece::Empty
    }

    pub fn moves(&self, location: Location, board: Board) -> Vec<Move> {
        match self {
            Self::Empty => Vec::new(),

            Piece::BlackPawn => {}
            Piece::WhitePawn => {}

            Piece::BlackBishop => {}
            Piece::WhiteBishop => {}

            Piece::BlackKnight => {}
            Piece::WhiteKnight => {}

            Piece::BlackRook => {}
            Piece::WhiteRook => {}

            Piece::BlackKing => {}
            Piece::WhiteKing => {}

            Piece::BlackQueen => {}
            Piece::WhiteQueen => {}
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Piece::Empty => Empty,
            Piece::WhitePawn => White,
            Piece::WhiteBishop => White,
            Piece::WhiteKnight => White,
            Piece::WhiteRook => White,
            Piece::WhiteKing => White,
            Piece::WhiteQueen => White,
            _ => Black,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Color {
    Black,
    White,
    Empty,
}

fn pawn_moves(location: Location, board: Board) -> Vec<Move> {

}

fn bishop_moves(location: Location, board: Board) -> Vec<Move> {

}

fn knight_moves(location: Location, board: Board) -> Vec<Move> {

}

fn rook_moves(location: Location, board: Board) -> Vec<Move> {
    let mut moves = Vec::new();

    let our_color = board.piece_at(location).color();

    for x in (location.x + 1)..8 {
        let other = (x, location.y).into();
        let piece = board.piece_at(other);
        if !piece.is_empty() {
            if piece.color() != our_color {
                moves.push((location, other).into());
            }

            break;
        }

        moves.push((location, other).into());
    }

    for x in 0..location.x {
        moves.push((location, (x, location.y).into()).into());
    }

    moves
}

fn king_moves(location: Location, board: Board) -> Vec<Move> {

}

fn queen_moves(location: Location, board: Board) -> Vec<Move> {

}