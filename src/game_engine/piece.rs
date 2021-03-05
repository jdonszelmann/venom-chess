use crate::game_engine::color::Color::{White, EmptyColor, Black};
use std::fmt;
use crate::game_engine::color::Color;

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
        self == &Piece::Empty
    }

    pub fn is_pawn(&self) -> bool {
        self == &Piece::BlackPawn || self == &Piece::WhitePawn
    }

    pub fn is_bishop(&self) -> bool {
        self == &Piece::BlackBishop || self == &Piece::WhiteBishop
    }

    pub fn is_knight(&self) -> bool {
        self == &Piece::BlackKnight || self == &Piece::WhiteKnight
    }

    pub fn is_rook(&self) -> bool {
        self == &Piece::BlackRook || self == &Piece::WhiteRook
    }

    pub fn is_king(&self) -> bool {
        self == &Piece::BlackKing || self == &Piece::WhiteKing
    }

    pub fn is_queen(&self) -> bool {
        self == &Piece::BlackQueen || self == &Piece::WhiteQueen
    }

    pub fn color(&self) -> Color {
        match self {
            Piece::Empty => EmptyColor,
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

pub fn knight_of_color(color : Color) -> Piece {
    if color == White {
        Piece::WhiteKnight
    } else {
        Piece::BlackKnight
    }
}

pub fn bishop_of_color(color : Color) -> Piece {
    if color == White {
        Piece::WhiteBishop
    } else {
        Piece::BlackBishop
    }
}

pub fn rook_of_color(color : Color) -> Piece {
    if color == White {
        Piece::WhiteRook
    } else {
        Piece::BlackRook
    }
}

pub fn queen_of_color(color : Color) -> Piece {
    if color == White {
        Piece::WhiteQueen
    } else {
        Piece::BlackQueen
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.color() == Color::White {
            write!(f, "\x1b[97m")?;
        } else {
            write!(f, "\x1b[30m")?;
        }

        match self {
            Piece::Empty => write!(f, "   "),
            Piece::BlackPawn => write!(f, " ♟ "),
            Piece::WhitePawn => write!(f, " ♙ "),
            Piece::BlackBishop => write!(f, " ♝ "),
            Piece::WhiteBishop => write!(f, " ♗ "),
            Piece::BlackKnight => write!(f, " ♞ "),
            Piece::WhiteKnight => write!(f, " ♘ "),
            Piece::BlackRook => write!(f, " ♜ "),
            Piece::WhiteRook => write!(f, " ♖ "),
            Piece::BlackKing => write!(f, " ♚ "),
            Piece::WhiteKing => write!(f, " ♔ "),
            Piece::BlackQueen => write!(f, " ♛ "),
            Piece::WhiteQueen => write!(f, " ♕ "),
        }?;

        Ok(())
    }
}

