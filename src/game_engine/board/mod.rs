pub use basic::BasicBoard;

use crate::game_engine::chess_move::{Location, Move};
use crate::game_engine::color::Color;
use crate::game_engine::piece::Piece;
use std::hash::Hash;

pub mod basic;
pub mod zobrist;
pub mod pst;
pub mod display;

pub trait Board: Sized + Clone + Hash + Eq {
    fn moves(&self, location: impl Into<Location>) -> Vec<Move>;
    fn all_moves(&self) -> Vec<Move>;

    #[inline]
    fn transition(&self, m: Move) -> Self {
        self.transition_with_move_func(m, |_, _, _, _|{})
    }
    fn transition_with_move_func(&self, m: Move, func: impl FnMut(Piece, Location, Location, Piece)) -> Self;

    fn all_pieces(&self) -> Vec<(Piece, Location)>;

    fn is_terminal(&self) -> Option<Color>;

    fn current_player(&self) -> Color;

    fn get_castling_rights(&self) -> [bool; 4];
    fn get_en_passant(&self) -> i8;
    fn get_material_score(&self) -> i32;
    fn piece_at(&self, l: impl Into<Location>) -> Piece;
    fn piece_at_mut(&mut self, l: impl Into<Location>) -> &mut Piece;
}

