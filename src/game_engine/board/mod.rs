pub use basic::BasicBoard;

use crate::game_engine::chess_move::{Location, Move};
use crate::game_engine::color::Color;
use crate::game_engine::piece::Piece;
use std::time::Duration;

pub mod basic;
pub mod zobrist;
pub mod pst;
pub mod display;

pub trait Board: Sized + Clone {
    fn moves(&self, location: impl Into<Location>) -> Vec<Move>;
    fn all_moves(&self) -> Vec<Move>;

    #[inline]
    fn transition(&self, m: Move) -> Self {
        self.transition_with_move_func(m, |_, _|{}, |_, _|{})
    }
    fn transition_with_move_func(
        &self,
        m: Move,
        remove_piece: impl FnMut(Piece, Location),
        add_piece: impl FnMut(Piece, Location),
    ) -> Self;

    fn all_pieces(&self) -> Vec<(Piece, Location)>;

    fn is_terminal(&self) -> Option<Color>;

    fn current_player(&self) -> Color;

    fn get_castling_rights(&self) -> [bool; 4];
    fn get_en_passant(&self) -> i8;
    fn piece_at(&self, l: impl Into<Location>) -> Piece;
    fn piece_at_mut(&mut self, l: impl Into<Location>) -> &mut Piece;
    fn get_clock(&self) -> [Duration; 2];
    fn set_clock(&mut self, time : Duration);

    fn get_material_score(&self) -> i32;
    fn heuristic(&self) -> f64;

    // TODO: use built in hash trait
    fn hash(&self) -> u64;
}

