use crate::game_engine::piece::{Piece, Color};
use crate::game_engine::piece::Piece::*;
use crate::game_engine::chessMove::{Move, Location};
use std::fmt;
use crate::game_engine::piece::Color::{White, Black};
use std::io::SeekFrom::Current;


#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct Board {
    pub board: [[Piece; 8]; 8],

    pub current: Color,
}


impl Board {
    pub const DEFAULT_BOARD: Board = Board {
        board: [
            [BlackRook, BlackKnight, BlackBishop, BlackQueen, BlackKing, BlackBishop, BlackKnight, BlackRook],
            [BlackPawn, BlackPawn, BlackPawn, BlackPawn, BlackPawn, BlackPawn, BlackPawn, BlackPawn],
            [Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
            [Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
            [Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
            [Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
            [WhitePawn, WhitePawn, WhitePawn, WhitePawn, WhitePawn, WhitePawn, WhitePawn, WhitePawn],
            [WhiteRook, WhiteKnight, WhiteBishop, WhiteQueen, WhiteKing, WhiteBishop, WhiteKnight, WhiteRook],
        ],

        current: White,
    };

    pub fn new() -> Self {
        Self {
            board: [[Empty; 8]; 8],
            current: White,
        }
    }

    pub fn possible_moves(&self) -> Vec<Move> {
        todo!()
    }

    pub fn transition(&self, m: Move) -> Self {
        let mut new_board = self.clone();
        let movable = self.piece_at(m.from);

        *new_board.piece_at_mut(m.to) = movable;
        *new_board.piece_at_mut(m.from)= Piece::Empty;

        new_board.current = self.current.other();

        new_board
    }

    pub fn piece_at(&self, l: Location) -> Piece {
        self.board[l.y as usize][l.x as usize]
    }

    pub fn piece_at_mut(&mut self, l: Location) -> &mut Piece {
        &mut self.board[l.y as usize][l.x as usize]
    }

    pub fn king_location(&self, color: Color) -> Option<Location> {
        if color == Color::White {
            for y in 0..8 {
                for x in 0..8 {
                    let other = self.piece_at((x, y).into());
                    if other == Piece::WhiteKing{
                        return Some((x,y).into());
                    }
                }
            }
        } else {
            for y in 0..8 {
                for x in 0..8 {
                    let other = self.piece_at((x, y).into());
                    if other == Piece::BlackKing{
                        return Some((x,y).into());
                    }
                }
            }
        }
        None
    }

    pub fn king_check(&self, color: Color) -> bool{

        let king_loc = self.king_location(color).unwrap();

        for off in 1..(king_loc.x.min(king_loc.y)) {
            let other = (king_loc.x - off, king_loc.y - off).into();
            let piece = self.piece_at(other);
            if !piece.is_empty() {
                if piece.color() != color {
                    if piece.is_bishop() || piece.is_queen(){
                        return true
                    }
                }
                break;
            }
        }

        for off in 1..((7 - king_loc.x).min(king_loc.y)) {
            let other = (king_loc.x + off, king_loc.y - off).into();
            let piece = self.piece_at(other);
            if !piece.is_empty() {
                if piece.color() != color {
                    if piece.is_bishop() || piece.is_queen(){
                        return true
                    }
                }
                break;
            }
        }

        for off in 1..((7 - king_loc.x).min(7 - king_loc.y)) {
            let other = (king_loc.x + off, king_loc.y + off).into();
            let piece = self.piece_at(other);
            if !piece.is_empty() {
                if piece.color() != color {
                    if piece.is_bishop() || piece.is_queen(){
                        return true
                    }
                }
                break;
            }
        }

        for off in 1..(king_loc.x.min(7 - king_loc.y)) {
            let other = (king_loc.x - off, king_loc.y + off).into();
            let piece = self.piece_at(other);
            if !piece.is_empty() {
                if piece.color() != color {
                    if piece.is_bishop() || piece.is_queen(){
                        return true
                    }
                }
                break;
            }
        }





        for x in (king_loc.x + 1)..8 {
            let other = (x, king_loc.y).into();
            let piece = self.piece_at(other);
            if !piece.is_empty() {
                if piece.color() != color {
                    if piece.is_rook() || piece.is_queen(){
                        return true
                    }
                }
                break;
            }
        }

        for x in (0..king_loc.x).rev() {
            let other = (x, king_loc.y).into();
            let piece = self.piece_at(other);
            if !piece.is_empty() {
                if piece.color() != color {
                    if piece.is_rook() || piece.is_queen(){
                        return true
                    }
                }
                break;
            }
        }

        for y in (king_loc.y + 1)..8 {
            let other = (king_loc.x, y).into();
            let piece = self.piece_at(other);
            if !piece.is_empty() {
                if piece.color() != color {
                    if piece.is_rook() || piece.is_queen(){
                        return true
                    }
                }
                break;
            }
        }

        for y in (0..king_loc.y).rev() {
            let other = (king_loc.x, y).into();
            let piece = self.piece_at(other);
            if !piece.is_empty() {
                if piece.color() != color {
                    if piece.is_rook() || piece.is_queen(){
                        return true
                    }
                }
                break;
            }
        }



        for (x, y) in &[(king_loc.x + 2, king_loc.y + 1), (king_loc.x + 2, king_loc.y - 1),
            (king_loc.x - 2, king_loc.y + 1), (king_loc.x - 2, king_loc.y - 1),
            (king_loc.x + 1, king_loc.y + 2), (king_loc.x - 1, king_loc.y + 2),
            (king_loc.x + 1, king_loc.y - 2), (king_loc.x - 1, king_loc.y - 2)] {
            if *x < 0 || *x >= 8 {
                continue;
            }

            if *y < 0 || *y >= 8 {
                continue;
            }

            let l = (*x, *y).into();

            let piece = self.piece_at(l);
            if !piece.is_empty() {
                if piece.color() != color {
                    if piece.is_knight() {
                        return true
                    }
                }
                break;
            }
        }

        for (x, y) in &[(king_loc.x + 1, king_loc.y), (king_loc.x + 1, king_loc.y + 1),
            (king_loc.x, king_loc.y + 1), (king_loc.x - 1, king_loc.y + 1),
            (king_loc.x - 1, king_loc.y), (king_loc.x - 1, king_loc.y - 1),
            (king_loc.x, king_loc.y - 1), (king_loc.x + 1, king_loc.y - 1)] {
            if *x < 0 || *x >= 8 {
                continue;
            }

            if *y < 0 || *y >= 8 {
                continue;
            }

            let l = (*x, *y).into();

            let piece = self.piece_at(l);
            if !piece.is_empty() {
                if piece.color() != color {
                    if piece.is_king() {
                        return true
                    }
                }
                break;
            }

        }

        if color == Black{
            if king_loc.y<7 {
                if king_loc.x > 0 {
                    let other = (king_loc.x-1, king_loc.y + 1).into();
                    let piece = self.piece_at(other);
                    if !piece.is_empty() {
                        if piece.color() != color {
                            if piece == WhitePawn {
                                return true
                            }
                        }
                    }
                }
                if king_loc.x<7{
                    let other = (king_loc.x+1, king_loc.y + 1).into();
                    let piece = self.piece_at(other);
                    if !piece.is_empty() {
                        if piece.color() != color {
                            if piece == WhitePawn {
                                return true
                            }
                        }
                    }
                }
            }
        } else {
            if king_loc.y>0 {
                if king_loc.x > 0 {
                    let other = (king_loc.x-1, king_loc.y - 1).into();
                    let piece = self.piece_at(other);
                    if !piece.is_empty() {
                        if piece.color() != color {
                            if piece == BlackPawn {
                                return true
                            }
                        }
                    }
                }
                if king_loc.x<7{
                    let other = (king_loc.x+1, king_loc.y - 1).into();
                    let piece = self.piece_at(other);
                    if !piece.is_empty() {
                        if piece.color() != color {
                            if piece == BlackPawn {
                                return true
                            }
                        }
                    }
                }
            }
        }

        false
    }

}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..8 {
            write!(f, "{} ", y)?;

            for x in 0..8 {

                if (x + y) % 2 == 0{
                    write!(f, "\x1b[100m")?;
                } else {
                    write!(f, "\x1b[45m")?;
                }

                write!(f, "{}", self.piece_at((x, y).into()))?;
                write!(f, "\x1b[0m")?;

            }
            writeln!(f);
        }

        writeln!(f, "   0  1  2  3  4  5  6  7 ")?;
        if self.current == Color::White {
            writeln!(f, "current player: White")?;
        } else {
            writeln!(f, "current player: Black")?;
        }

        Ok(())
    }
}