pub use basic::BasicBoard;

use crate::game_engine::chess_move::{Location, Move};
use crate::game_engine::color::Color;
use crate::game_engine::piece::Piece;

pub mod basic;

pub trait Board: Sized + Clone {
    fn moves(&self, location: impl Into<Location>) -> Vec<Move>;
    fn all_moves(&self) -> Vec<Move>;
    fn transition(&self, m: Move) -> Self;

    fn all_pieces(&self) -> Vec<(Piece, Location)>;

    fn is_terminal(&self) -> Option<Color>;

    fn get_castling_rights(&self) -> [bool; 4];
    fn piece_at(&self, l: impl Into<Location>) -> Piece;
    fn piece_at_mut(&mut self, l: impl Into<Location>) -> &mut Piece;
}

