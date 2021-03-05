pub use basic::BasicBoard;

use crate::game_engine::chess_move::{Location, Move};
use crate::game_engine::color::Color;
use crate::game_engine::piece::Piece;

pub mod basic;
pub mod zobrist;

pub trait Board: Sized {
    fn moves(&self, location: impl Into<Location>) -> Vec<Move>;
    fn all_moves(&self) -> Vec<Move>;

    #[inline]
    fn transition(&self, m: Move) -> Self {
        self.transition_with_move_func(m, |_, _, _, _|{})
    }
    fn transition_with_move_func(&self, m: Move, func: impl FnMut(Piece, Location, Piece, Location)) -> Self;

    fn all_pieces(&self) -> Vec<(Piece, Location)>;

    fn is_terminal(&self) -> Option<Color>;

    fn current_player(&self) -> Color;

    fn piece_at(&self, l: impl Into<Location>) -> Piece;
    fn piece_at_mut(&mut self, l: impl Into<Location>) -> &mut Piece;
}

