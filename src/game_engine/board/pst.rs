use crate::game_engine::board::Board;
use crate::game_engine::chess_move::{Move, Location};
use crate::game_engine::piece::Piece;
use crate::game_engine::color::Color;
use std::fmt;


#[derive(Clone, Eq, PartialEq, Debug,)]
pub struct ZobristBoard<B> {
    inner: B,
    heuristic_value: u64
}

impl<B: fmt::Display> fmt::Display for ZobristBoard<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl<B: Board> ZobristBoard<B> {
    pub fn new(inner: B) -> Self {


        Self {
            inner,
            heuristic_value: 0,
        }
    }
}
//
// impl<B> Board for ZobristBoard<B> where B: Board {
//     #[inline]
//     fn moves(&self, location: impl Into<Location>) -> Vec<Move> {
//         self.inner.moves(location)
//     }
//
//     #[inline]
//     fn all_moves(&self) -> Vec<Move> {
//         self.inner.all_moves()
//     }
//
//     fn transition_with_move_func(&self, m: Move, mut func: impl FnMut(Piece, Location, Location, Piece)) -> Self {
//
//         let mut previous_hv = self.heuristic_value;
//
//         let inner = self.inner.transition_with_move_func(m, |p, f, t, r| {
//
//
//
//             func(p, f, t, r);
//         });
//
//
//         Self {
//             inner,
//             heuristic_value: previous_hv,
//         }
//     }
//
//     #[inline]
//     fn all_pieces(&self) -> Vec<(Piece, Location)> {
//         self.inner.all_pieces()
//     }
//
//     #[inline]
//     fn is_terminal(&self) -> Option<Color> {
//         self.inner.is_terminal()
//     }
//
//     #[inline]
//     fn current_player(&self) -> Color {
//         self.inner.current_player()
//     }
//
//     #[inline]
//     fn get_castling_rights(&self) -> [bool; 4] {
//         self.inner.get_castling_rights()
//     }
//
//     #[inline]
//     fn get_en_passant(&self) -> i8 {
//         self.inner.get_en_passant()
//     }
//
//     #[inline]
//     fn get_material_score(&self) -> i32 {
//         self.inner.get_material_score()
//     }
//
//     #[inline]
//     fn piece_at(&self, l: impl Into<Location>) -> Piece {
//         self.inner.piece_at(l)
//     }
//
//     #[inline]
//     fn piece_at_mut(&mut self, l: impl Into<Location>) -> &mut Piece {
//         self.inner.piece_at_mut(l)
//     }
// }
