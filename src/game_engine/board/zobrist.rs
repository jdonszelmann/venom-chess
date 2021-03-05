use crate::game_engine::board::Board;
use crate::game_engine::chess_move::{Move, Location};
use crate::game_engine::piece::Piece;
use crate::game_engine::color::Color;
use std::hash::{Hash, Hasher};
use rand::rngs::StdRng;
use rand::{SeedableRng, RngCore, Rng};
use lazy_static::lazy_static;
use std::fmt;

pub struct ZobristKeys {
    pieces: [[[u64; 12]; 8]; 8],
    color: u64,
    castling: [u64; 4],
    en_passant: [u64; 8],
}

impl ZobristKeys {
    pub fn new(seed: usize) -> Self {
        let mut seed_bytes = [0u8; 32];
        for (index, i) in seed_bytes.iter_mut().enumerate() {
            *i = seed.to_be_bytes()[index % 8];
        }
        let mut rng = StdRng::from_seed(seed_bytes);

        let mut pieces = [[[0; 12]; 8]; 8];
        let mut castling = [0; 4];
        let mut en_passant = [0; 8];

        let color = rng.gen();

        for i in 0..8 {
            for j in 0..8 {
                rng.fill(&mut pieces[j][i]);
            }
        }
        rng.fill(&mut castling);
        rng.fill(&mut en_passant);

        Self {
            pieces,
            color,
            castling,
            en_passant,
        }
    }

    pub fn move_piece(&self, mut hash: u64, piece_type: Piece, from: Location, to: Location, replaces: Piece) -> u64 {
        if !piece_type.is_empty() {
            hash = hash ^ self.pieces[from.y as usize][from.x as usize][piece_type.to_number()];
            hash = hash ^ self.pieces[to.y as usize][to.x as usize][piece_type.to_number()];
        }
        if !replaces.is_empty() {
            hash = hash ^ self.pieces[from.y as usize][from.x as usize][replaces.to_number()];
            hash = hash ^ self.pieces[to.y as usize][to.x as usize][replaces.to_number()];
        }

        hash
    }

    pub fn apply_castling(&self, mut hash: u64, apply_castling: [bool; 4]) -> u64 {
        for i in 0..4 {
            if apply_castling[i] {
                hash = hash ^ self.castling[i];
            }
        }
        hash
    }

    pub fn update_castling(&self, hash: u64, previous_rights: [bool; 4], rights: [bool; 4]) -> u64 {
        let hash = self.apply_castling(hash, previous_rights);
        let hash = self.apply_castling(hash, rights);
        hash
    }

    pub fn apply_en_passant(&self, hash: u64, apply_en_passant: i8) -> u64 {
        if apply_en_passant >= 8 || apply_en_passant < 0 {
            hash
        } else {
            hash ^ self.en_passant[apply_en_passant as usize]
        }
    }

    pub fn update_en_passant(&self, hash: u64, previous_rights: i8, rights: i8) -> u64 {
        let hash = self.apply_en_passant(hash, previous_rights);
        let hash = self.apply_en_passant(hash, rights);
        hash
    }

    pub fn switch_color(&self, hash: u64) -> u64 {
        hash ^ self.color
    }
}

lazy_static!(static ref ZOBRIST_KEYS: ZobristKeys = ZobristKeys::new(10););


#[derive(Clone, Eq, PartialEq, Debug)]
pub struct ZobristBoard<B> {
    inner: B,
    hash: u64
}

impl<B: fmt::Display> fmt::Display for ZobristBoard<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl<B: Board> ZobristBoard<B> {
    pub fn new(inner: B) -> Self {
        let mut hash = 0;

        for i in 0..8 {
            for j in 0..8 {
                let location: Location = (i, j).into();
                let piece = inner.piece_at(location);
                hash = ZOBRIST_KEYS.move_piece(hash, piece, location, location, Piece::Empty)
            }
        }

        // the hash begins at "white"
        if inner.current_player() == Color::Black {
            hash = ZOBRIST_KEYS.switch_color(hash);
        }

        hash = ZOBRIST_KEYS.update_castling(hash, [false; 4], inner.get_castling_rights());
        hash = ZOBRIST_KEYS.update_en_passant(hash, 8, inner.get_en_passant());


        Self {
            inner,
            hash,
        }
    }
}

impl<B> Hash for ZobristBoard<B> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.hash)
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
        let mut hash = self.hash;

        let inner = self.inner.transition_with_move_func(m, |p, f, t, r| {
            hash = ZOBRIST_KEYS.move_piece(hash, p, f, t, r);
            func(p, f, t, r);
        });

        hash = ZOBRIST_KEYS.switch_color(hash);
        hash = ZOBRIST_KEYS.update_en_passant(hash, self.get_en_passant(), inner.get_en_passant());
        hash = ZOBRIST_KEYS.update_castling(hash, self.get_castling_rights(), inner.get_castling_rights());

        Self {
            inner,
            hash,
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

    fn get_material_score(&self) -> i8 {
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
    use crate::ai::random_play::RandomPlay;

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