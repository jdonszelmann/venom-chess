use crate::game_engine::board::Board;
use crate::game_engine::chessMove::{Location, Move};
use crate::game_engine::piece::Color::{White, Empty, Black};
use std::fmt;

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

            Piece::BlackBishop => bishop_moves(location, board),
            Piece::WhiteBishop => bishop_moves(location, board),

            Piece::BlackKnight => knight_moves(location, board),
            Piece::WhiteKnight => knight_moves(location, board),

            Piece::BlackRook => rook_moves(location, board),
            Piece::WhiteRook => rook_moves(location, board),

            Piece::BlackKing => king_moves(location, board),
            Piece::WhiteKing => king_moves(location, board),

            Piece::BlackQueen => queen_moves(location, board),
            Piece::WhiteQueen => queen_moves(location, board),
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

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.color() == Color::White {
            write!(f, "\033[97m")?;
        } else {
            write!(f, "\033[30m")?;
        }

        match self {
            Piece::Empty => write!(f, " "),
            Piece::BlackPawn => write!(f, "♟︎"),
            Piece::WhitePawn => write!(f, "♙"),
            Piece::BlackBishop => write!(f, "♝"),
            Piece::WhiteBishop => write!(f, "♗"),
            Piece::BlackKnight => write!(f, "♞"),
            Piece::WhiteKnight => write!(f, "♘"),
            Piece::BlackRook => write!(f, "♜"),
            Piece::WhiteRook => write!(f, "♖"),
            Piece::BlackKing => write!(f, "♚"),
            Piece::WhiteKing => write!(f, "♔"),
            Piece::BlackQueen => write!(f, "♛"),
            Piece::WhiteQueen => write!(f, "♕"),
        }?;

        Ok(())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Color {
    Black,
    White,
    Empty,
}

fn pawn_moves(location: Location, board: Board) -> Vec<Move> {}

fn bishop_moves(location: Location, board: Board) -> Vec<Move> {
    let mut moves = Vec::new();
    let our_color = board.piece_at(location).color();

    for off in 1..(location.x.min(location.y)) {
        let other = (location.x - off, location.y - off).into();
        let piece = board.piece_at(other);
        if !piece.is_empty() {
            if piece.color() != our_color {
                moves.push((location, other).into());
            }
            break;
        }
        moves.push((location, other).into());
    }

    for off in 1..((7 - location.x).min(location.y)) {
        let other = (location.x + off, location.y - off).into();
        let piece = board.piece_at(other);
        if !piece.is_empty() {
            if piece.color() != our_color {
                moves.push((location, other).into());
            }
            break;
        }
        moves.push((location, other).into());
    }

    for off in 1..((7 - location.x).min(7 - location.y)) {
        let other = (location.x + off, location.y + off).into();
        let piece = board.piece_at(other);
        if !piece.is_empty() {
            if piece.color() != our_color {
                moves.push((location, other).into());
            }
            break;
        }
        moves.push((location, other).into());
    }

    for off in 1..(location.x.min(7 - location.y)) {
        let other = (location.x - off, location.y + off).into();
        let piece = board.piece_at(other);
        if !piece.is_empty() {
            if piece.color() != our_color {
                moves.push((location, other).into());
            }
            break;
        }
        moves.push((location, other).into());
    }

    moves
}

fn knight_moves(location: Location, board: Board) -> Vec<Move> {}

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

    for y in (location.y + 1)..8 {
        let other = (location.x, y).into();
        let piece = board.piece_at(other);
        if !piece.is_empty() {
            if piece.color() != our_color {
                moves.push((location, other).into());
            }

            break;
        }

        moves.push((location, other).into());
    }

    for y in 0..location.y {
        let other = (location.x, y).into();
        let piece = board.piece_at(other);
        if !piece.is_empty() {
            if piece.color() != our_color {
                moves.push((location, other).into());
            }

            break;
        }

        moves.push((location, other).into());
    }

    moves
}

fn king_moves(location: Location, board: Board) -> Vec<Move> {
    let mut moves = Vec::new();
    let our_color = board.piece_at(location).color();

    for (x, y) in [(x + 1, y), (x + 1, y + 1), (x, y + 1), (x - 1, y + 1), (x - 1, y), (x - 1, y - 1), (x, y - 1), (x + 1, y - 1)] {
        let l = (x, y).into();

        if x < 0 || x >= 8 {
            continue
        }

        if y < 0 || y >= 8 {
            continue
        }

        let piece = board.piece_at(l);
        if !piece.is_empty() && piece.color() == our_color {
            continue
        }

        moves.push((location, l).into());
    }

    moves
}

fn queen_moves(location: Location, board: Board) -> Vec<Move> {}