use crate::game_engine::piece::{Piece, knight_of_color, bishop_of_color, rook_of_color, queen_of_color};
use crate::game_engine::color::Color;

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub struct Location {
    pub x: i8,
    pub y: i8,
}

impl Location {
    pub fn new(x: i8, y: i8) -> Self {
        assert!(x >= 0 && x < 8);
        assert!(y >= 0 && y < 8);

        Self {
            x, y
        }
    }
}

impl From<(i8, i8)> for Location {
    fn from((x, y): (i8, i8)) -> Self {
        Self::new(x, y)
    }
}


#[repr(u8)]
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum Extra {
    Quiet = 0,
    DoublePawn = 1,
    KingCastle = 2,
    QueenCastle = 3,
    Captures = 4,
    EnPassantCaptures = 5,

    KnightPromotion = 8,
    BishopPromotion = 9,
    RookPromotion = 10,
    QueenPromotion = 11,

    KnightPromotionCapture = 12,
    BishopPromotionCapture = 13,
    RookPromotionCapture = 14,
    QueenPromotionCapture = 15,
}

impl Extra {
    pub fn is_promotion(&self) -> bool {
        self.is_capturing_promotion() || self.is_non_capturing_promotion()
    }

    pub fn is_non_capturing_promotion(&self) -> bool {
        match self {
            Extra::KnightPromotion => true,
            Extra::BishopPromotion => true,
            Extra::RookPromotion => true,
            Extra::QueenPromotion => true,
            _ => false,
        }
    }

    pub fn is_capturing_promotion(&self) -> bool {
        match self {
            Extra::KnightPromotionCapture => true,
            Extra::BishopPromotionCapture => true,
            Extra::RookPromotionCapture => true,
            Extra::QueenPromotionCapture => true,
            _ => false,
        }
    }

    pub fn capturing_promotion(piece: Piece) -> Extra {
        match piece {
            Piece::Empty => panic!("can't promote empty"),
            Piece::BlackPawn => panic!("can't promote pawn"),
            Piece::WhitePawn => panic!("can't promote pawn"),
            Piece::BlackBishop => Extra::BishopPromotion,
            Piece::WhiteBishop => Extra::BishopPromotion,
            Piece::BlackKnight => Extra::KnightPromotion,
            Piece::WhiteKnight => Extra::KnightPromotion,
            Piece::BlackRook => Extra::RookPromotion,
            Piece::WhiteRook => Extra::RookPromotion,
            Piece::BlackKing => panic!("can't promote king"),
            Piece::WhiteKing => panic!("can't promote king"),
            Piece::BlackQueen => Extra::QueenPromotion,
            Piece::WhiteQueen => Extra::QueenPromotion,
        }
    }

    pub fn promotion_of_color(&self, color: Color) -> Option<Piece> {
        match self {
            Extra::Quiet => None,
            Extra::DoublePawn => None,
            Extra::KingCastle => None,
            Extra::QueenCastle => None,
            Extra::Captures => None,
            Extra::EnPassantCaptures => None,
            Extra::KnightPromotion => Some(knight_of_color(color)),
            Extra::BishopPromotion => Some(bishop_of_color(color)),
            Extra::RookPromotion => Some(rook_of_color(color)),
            Extra::QueenPromotion => Some(queen_of_color(color)),
            Extra::KnightPromotionCapture => Some(knight_of_color(color)),
            Extra::BishopPromotionCapture => Some(bishop_of_color(color)),
            Extra::RookPromotionCapture => Some(rook_of_color(color)),
            Extra::QueenPromotionCapture => Some(queen_of_color(color)),
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub struct Move {
    pub from: Location,
    pub to: Location,
    pub extra: Extra,
}

impl Move {
    pub fn new(from: Location, to: Location, extra: Extra) -> Self {
        Self {
            from,
            to,
            extra,
        }
    }
}

impl From<(Location, Location)> for Move {
    fn from((from, to): (Location, Location)) -> Self {
        Self {
            from, to, extra: Extra::Quiet,
        }
    }
}

impl From<(Location, Location, Extra)> for Move {
    fn from((from, to, extra): (Location, Location, Extra)) -> Self {
        Self {
            from, to, extra,
        }
    }
}

impl From<((i8, i8), (i8, i8))> for Move {
    fn from((from, to): ((i8, i8), (i8, i8))) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
            extra: Extra::Quiet
        }
    }
}

impl From<((i8, i8), (i8, i8), Extra)> for Move {
    fn from((from, to, extra): ((i8, i8), (i8, i8), Extra)) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
            extra,
        }
    }
}