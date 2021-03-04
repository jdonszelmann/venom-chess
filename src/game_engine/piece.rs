use crate::game_engine::board::Board;
use crate::game_engine::chess_move::{Location, Move};
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
        self == &Piece::Empty
    }

    pub fn is_pawn(&self) -> bool {
        self == &Piece::BlackPawn || self == &Piece::WhitePawn
    }

    pub fn is_bishop(&self) -> bool {
        self == &Piece::BlackBishop || self == &Piece::WhiteBishop
    }

    pub fn is_knight(&self) -> bool {
        self == &Piece::BlackKnight || self == &Piece::WhiteKing
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

    pub fn moves(&self, location: Location, board: &Board) -> Vec<Move> {
        match self {
            Self::Empty => Vec::new(),

            Piece::BlackPawn => pawn_moves_black(location, board),
            Piece::WhitePawn => pawn_moves_white(location, board),

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

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Color {
    Black,
    White,
    Empty,
}

impl Color {
    pub fn other(&self) -> Self {
        match self {
            Black => White,
            White => Black,
            Empty => Empty,
        }
    }
}

fn pawn_moves_black(location: Location, board: &Board) -> Vec<Move> {
    let mut moves = Vec::new();
    let our_color = board.piece_at(location).color();


    if location.y == 7 {
        return moves
    }

    let in_front = (location.x, location.y + 1).into();
    if board.piece_at(in_front).is_empty(){
        moves.push((location, in_front).into());

        if location.y == 1 {
            let double_in_front = (location.x, location.y + 2).into();
            if board.piece_at(double_in_front).is_empty(){
                moves.push((location, double_in_front).into());
            }
        }
    }

    if location.x>0{
        let other = (location.x-1, location.y + 1).into();
        let piece = board.piece_at(other);
        if !piece.is_empty() {
            if piece.color() != our_color {
                moves.push((location, other).into());
            }
        }
    }

    if location.x<7{
        let other = (location.x+1, location.y + 1).into();
        let piece = board.piece_at(other);
        if !piece.is_empty() {
            if piece.color() != our_color {
                moves.push((location, other).into());
            }
        }
    }

    moves
}

fn pawn_moves_white(location: Location, board: &Board) -> Vec<Move> {
    let mut moves = Vec::new();
    let our_color = board.piece_at(location).color();


    if location.y == 0 {
        return moves
    }

    let in_front = (location.x, location.y - 1).into();
    if board.piece_at(in_front).is_empty(){
        moves.push((location, in_front).into());

        if location.y == 6 {
            let double_in_front = (location.x, location.y - 2).into();
            if board.piece_at(double_in_front).is_empty(){
                moves.push((location, double_in_front).into());
            }
        }
    }

    if location.x>0{
        let other = (location.x-1, location.y - 1).into();
        let piece = board.piece_at(other);
        if !piece.is_empty() {
            if piece.color() != our_color {
                moves.push((location, other).into());
            }
        }
    }

    if location.x<7{
        let other = (location.x+1, location.y - 1).into();
        let piece = board.piece_at(other);
        if !piece.is_empty() {
            if piece.color() != our_color {
                moves.push((location, other).into());
            }
        }
    }

    moves
}



fn bishop_moves(location: Location, board: &Board) -> Vec<Move> {
    let mut moves = Vec::new();
    let our_color = board.piece_at(location).color();

    for off in 1..(location.x.min(location.y) + 1) {
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

    for off in 1..((7 - location.x).min(location.y) + 1) {
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

    for off in 1..((7 - location.x).min(7 - location.y) + 1) {
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

    for off in 1..(location.x.min(7 - location.y) + 1) {
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

fn knight_moves(location: Location, board: &Board) -> Vec<Move> {
    let mut moves = Vec::new();
    let our_color = board.piece_at(location).color();

    for (x, y) in &[(location.x + 2, location.y + 1), (location.x + 2, location.y - 1),
        (location.x - 2, location.y + 1), (location.x - 2, location.y - 1),
        (location.x + 1, location.y + 2), (location.x - 1, location.y + 2),
        (location.x + 1, location.y - 2), (location.x - 1, location.y - 2)] {

        if *x < 0 || *x >= 8 {
            continue;
        }

        if *y < 0 || *y >= 8 {
            continue;
        }

        let l = (*x, *y).into();

        let piece = board.piece_at(l);
        if !piece.is_empty() && piece.color() == our_color {
            continue;
        }

        moves.push((location, l).into());
    }

    moves
}

fn rook_moves(location: Location, board: &Board) -> Vec<Move> {
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

    for x in (0..location.x).rev() {
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

    for y in (0..location.y).rev() {
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

fn king_moves(location: Location, board: &Board) -> Vec<Move> {
    let mut moves = Vec::new();
    let our_color = board.piece_at(location).color();

    for (x, y) in &[(location.x + 1, location.y), (location.x + 1, location.y + 1),
        (location.x, location.y + 1), (location.x - 1, location.y + 1),
        (location.x - 1, location.y), (location.x - 1, location.y - 1),
        (location.x, location.y - 1), (location.x + 1, location.y - 1)] {
        if *x < 0 || *x >= 8 {
            continue;
        }

        if *y < 0 || *y >= 8 {
            continue;
        }

        let l = (*x, *y).into();

        let piece = board.piece_at(l);
        if !piece.is_empty() && piece.color() == our_color {
            continue;
        }

        moves.push((location, l).into());
    }

    moves
}

fn queen_moves(location: Location, board: &Board) -> Vec<Move> {
    let mut moves = bishop_moves(location,board);
    moves.append(&mut rook_moves(location,board));
    moves
}