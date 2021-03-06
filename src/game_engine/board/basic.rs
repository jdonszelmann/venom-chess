
use crate::game_engine::chess_move::{Location, Move, Extra};
use crate::game_engine::piece::{Piece, knight_of_color, bishop_of_color, rook_of_color, queen_of_color};
use crate::game_engine::color::Color::*;
use crate::game_engine::color::Color;
use crate::game_engine::piece::Piece::*;
use std::fmt;
use crate::game_engine::board::Board;
use crate::game_engine::king_check::king_check;
use crate::game_engine::piece_moves::{pawn_moves_black, pawn_moves_white, bishop_moves, knight_moves, rook_moves, king_moves, queen_moves};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hasher, Hash};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct BasicBoard {
    pub board: [[Piece; 8]; 8],
    pub current: Color,
    pub castling_rights : [bool; 4],
    pub en_passant : i8,
    pub material_score : i32,
}


impl BasicBoard {
    pub const DEFAULT_BOARD: BasicBoard = BasicBoard {
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

        castling_rights : [true;4],

        en_passant: 8,

        material_score: 0,
    };

    pub fn new() -> Self {
        Self {
            board: [[Empty; 8]; 8],
            current: White,
            castling_rights : [true;4],
            en_passant: 8,
            material_score: 0,
        }
    }
}

impl Board for BasicBoard {

    fn moves(&self, location: impl Into<Location>) -> Vec<Move> {
        let location = location.into();
        match self.piece_at(location) {
            Piece::Empty => Vec::new(),

            Piece::BlackPawn => pawn_moves_black(location, self),
            Piece::WhitePawn => pawn_moves_white(location, self),

            Piece::BlackBishop => bishop_moves(location, self),
            Piece::WhiteBishop => bishop_moves(location, self),

            Piece::BlackKnight => knight_moves(location, self),
            Piece::WhiteKnight => knight_moves(location, self),

            Piece::BlackRook => rook_moves(location, self),
            Piece::WhiteRook => rook_moves(location, self),

            Piece::BlackKing => king_moves(location, self),
            Piece::WhiteKing => king_moves(location, self),

            Piece::BlackQueen => queen_moves(location, self),
            Piece::WhiteQueen => queen_moves(location, self),
        }.into_iter()
            .filter(move |&i| !king_check(&self.transition(i), self.current))
            .collect()
    }

    fn all_moves(&self) -> Vec<Move> {
        self.all_pieces()
            .into_iter()
            .map(move |(_, l)| self.moves(l))
            .flatten()
            .collect()
    }

    fn transition_with_move_func(&self, m: Move, mut func: impl FnMut(Piece, Location, Location, Piece)) -> Self {
        let mut new_board = self.clone();

        let movable = self.piece_at(m.from);
        let replaces = self.piece_at(m.to);

        new_board.material_score += self.piece_at(m.to).material_worth();

        if movable == BlackKing{
            new_board.castling_rights[0] = false;
            new_board.castling_rights[1] = false;
        }
        if movable == WhiteKing{
            new_board.castling_rights[2] = false;
            new_board.castling_rights[3] = false;
        }

        if m.from == (0,0).into(){
            new_board.castling_rights[0] = false;
        }
        if m.from == (7,0).into(){
            new_board.castling_rights[1] = false;
        }
        if m.from == (0,7).into(){
            new_board.castling_rights[2] = false;
        }
        if m.from == (7,7).into(){
            new_board.castling_rights[3] = false;
        }

        if m.to == (0,0).into(){
            new_board.castling_rights[0] = false;
        }
        if m.to == (7,0).into(){
            new_board.castling_rights[1] = false;
        }
        if m.to == (0,7).into(){
            new_board.castling_rights[2] = false;
        }
        if m.to == (7,7).into(){
            new_board.castling_rights[3] = false;
        }

        new_board.en_passant = 8;
        if movable == Piece::WhitePawn {
            if m.to.y + 2 == m.from.y {
                new_board.en_passant = m.from.x;
            }
        }

        if movable == Piece::BlackPawn {
            if m.to.y - 2 == m.from.y {
                new_board.en_passant = m.from.x;
            }
        }

        if movable == BlackKing {
            if m == ((4,0),(2,0)).into(){
                *new_board.piece_at_mut((3,0)) = Piece::BlackRook;
                *new_board.piece_at_mut((0,0))= Piece::Empty;

                func(Piece::BlackRook, (0,0).into(), (3,0).into(), Piece::Empty);
            }

            if m == ((4,0),(6,0)).into(){
                *new_board.piece_at_mut((5,0)) = Piece::BlackRook;
                *new_board.piece_at_mut((7,0))= Piece::Empty;

                func(Piece::BlackRook, (7,0).into(), (5,0).into(), Piece::Empty);
            }
        }

        if movable == BlackPawn && m.to.x != m.from.x{
            if self.piece_at(m.to).is_empty(){
                let l = (m.to.x,m.to.y-1);
                let old = new_board.piece_at(l);
                *new_board.piece_at_mut((m.to.x,m.to.y-1)) = Piece::Empty;

                func(Piece::Empty, l.into(), l.into(), old);
                new_board.material_score += old.material_worth();
            }
        }

        if movable == WhitePawn  && m.to.x != m.from.x{
            if self.piece_at(m.to).is_empty(){
                let l = (m.to.x,m.to.y+1);
                let old = new_board.piece_at(l);
                *new_board.piece_at_mut(l) = Piece::Empty;

                func(Piece::Empty, l.into(), l.into(), old);
                new_board.material_score += old.material_worth();
            }
        }

        if movable == WhiteKing {
            if m == ((4,7),(2,7)).into(){
                *new_board.piece_at_mut((3,7)) = Piece::WhiteRook;
                *new_board.piece_at_mut((0,7))= Piece::Empty;

                func(Piece::WhiteRook, (0,7).into(), (3,7).into(), Piece::Empty);
            }

            if m == ((4,7),(6,7)).into(){
                *new_board.piece_at_mut((5,7)) = Piece::WhiteRook;
                *new_board.piece_at_mut((7,7))= Piece::Empty;

                func(Piece::WhiteRook, (7,7).into(), (5,7).into(), Piece::Empty);
            }
        }

        let set_piece = match m.extra {
            Extra::KnightPromotion =>  knight_of_color(movable.color()),
            Extra::BishopPromotion => bishop_of_color(movable.color()),
            Extra::RookPromotion => rook_of_color(movable.color()),
            Extra::QueenPromotion => queen_of_color(movable.color()),
            _ => movable,
        };



        *new_board.piece_at_mut(m.to) = set_piece;
        *new_board.piece_at_mut(m.from)= Piece::Empty;

        func(movable, m.from, m.to, replaces);

        if m.extra.is_promotion() {
            new_board.material_score -= set_piece.material_worth();
            func(set_piece, m.to, m.to, movable);
        }

        new_board.current = self.current.other();

        new_board
    }


    fn all_pieces(&self) -> Vec<(Piece, Location)> {
        (0..8).map(move |i| (0..8).map(move |j| {
            let l = (i, j).into();
            (self.piece_at(l), l)
        }))
            .flatten()
            .filter(|(i, _)| !i.is_empty())
            .filter(move |(i, _)| i.color() == self.current)
            .collect()
    }

    fn is_terminal(&self) -> Option<Color> {
        if self.all_moves().len() == 0 {
            if king_check(self,self.current) {
                return Some(self.current.other());
            } else {
                return Some(Color::EmptyColor);
            }
        }

        None
    }

    fn current_player(&self) -> Color {
        self.current
    }

    fn piece_at(&self, l: impl Into<Location>) -> Piece {
        let l = l.into();
        self.board[l.y as usize][l.x as usize]
    }

    fn piece_at_mut(&mut self, l: impl Into<Location>) -> &mut Piece {
        let l = l.into();
        &mut self.board[l.y as usize][l.x as usize]
    }

    fn get_castling_rights(&self) -> [bool; 4] {
        self.castling_rights
    }

    fn get_en_passant(&self) -> i8 {
        self.en_passant
    }

    fn get_material_score(&self) -> i32 {
        self.material_score
    }

    fn hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.board.hash(&mut s);
        self.en_passant.hash(&mut s);
        self.current.hash(&mut s);
        self.castling_rights.hash(&mut s);
        s.finish()
    }
}

