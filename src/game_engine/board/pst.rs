use crate::game_engine::board::Board;
use crate::game_engine::chess_move::{Move, Location};
use crate::game_engine::piece::Piece;
use crate::game_engine::color::Color;
use std::hash::{Hash, Hasher};
use rand::rngs::StdRng;
use rand::{SeedableRng, RngCore, Rng};
use lazy_static::lazy_static;
use std::fmt;


#[derive(Clone, Eq, PartialEq, Debug)]
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

impl<B> Board for ZobristBoard<B> where B: Board {
    #[inline]
    fn moves(&self, location: impl Into<Location>) -> Vec<Move> {
        self.inner.moves(location)
    }

    #[inline]
    fn all_moves(&self) -> Vec<Move> {
        self.inner.all_moves()
    }

    fn transition_with_move_func(&self, m: Move, mut func: impl FnMut(Piece, Location, Location, Piece)) -> Self {

        let mut previous_hv = self.heuristic_value;

        let inner = self.inner.transition_with_move_func(m, |p, f, t, r| {



            func(p, f, t, r);
        });


        Self {
            inner,
            heuristic_value: previous_hv,
        }
    }

    #[inline]
    fn all_pieces(&self) -> Vec<(Piece, Location)> {
        self.inner.all_pieces()
    }

    #[inline]
    fn is_terminal(&self) -> Option<Color> {
        self.inner.is_terminal()
    }

    #[inline]
    fn current_player(&self) -> Color {
        self.inner.current_player()
    }

    #[inline]
    fn get_castling_rights(&self) -> [bool; 4] {
        self.inner.get_castling_rights()
    }

    #[inline]
    fn get_en_passant(&self) -> i8 {
        self.inner.get_en_passant()
    }

    fn get_material_score(&self) -> i32 {
        self.inner.get_material_score()
    }

    #[inline]
    fn piece_at(&self, l: impl Into<Location>) -> Piece {
        self.inner.piece_at(l)
    }

    #[inline]
    fn piece_at_mut(&mut self, l: impl Into<Location>) -> &mut Piece {
        self.inner.piece_at_mut(l)
    }
}

#[cfg(test)]
mod tests {
    use crate::game_engine::board::{BasicBoard, Board};
    use crate::game_engine::board::zobrist::{ZobristBoard, ZobristKeys};
    use super::ZOBRIST_KEYS;
    use crate::solver::random_play::RandomPlay;

    #[test]
    fn test_switch_color_twice() {
        let hash = 0;
        let hash = ZOBRIST_KEYS.switch_color(hash);
        assert_ne!(0, hash);
        let hash = ZOBRIST_KEYS.switch_color(hash);
        assert_eq!(0, hash);
    }

    #[test]
    fn test_hash_move_back() {
        let board = BasicBoard::DEFAULT_BOARD;
        let mut zboard = ZobristBoard::new(board);

        let initial_hash = zboard.hash;

        zboard = zboard.transition(((1, 7), (0, 5)).into());
        assert_ne!(initial_hash, zboard.hash);

        zboard = zboard.transition(((1, 0), (0, 2)).into());
        assert_ne!(initial_hash, zboard.hash);

        zboard = zboard.transition(((0, 5), (1, 7)).into());
        assert_ne!(initial_hash, zboard.hash);

        zboard = zboard.transition(((0, 2), (1, 0)).into());
        assert_eq!(initial_hash, zboard.hash);
    }

    #[test]
    fn test_hash_two_boards() {
        let board1 = BasicBoard::DEFAULT_BOARD;
        let board2 = BasicBoard::DEFAULT_BOARD;
        let zboard1 = ZobristBoard::new(board1);
        let zboard2 = ZobristBoard::new(board2);

        assert_eq!(zboard1.hash, zboard2.hash);
        assert_ne!(zboard1.hash, 0);
    }

    #[test]
    fn test_hash_fuzzer() {
        for _ in 0..10 {
            let mut board = BasicBoard::DEFAULT_BOARD;
            let mut zboard = ZobristBoard::new(board);
            let random_player = RandomPlay::new();

            for _ in 0..100 {
                zboard = match random_player.make_move(zboard.clone()) {
                    Some(i) => i,
                    None => break,
                }
            }

            let zboard2 = ZobristBoard::new(zboard.inner.clone());

            println!("{}", zboard2);
            println!("{}", zboard);


            assert_eq!(zboard.hash, zboard2.hash);
        }
    }
}