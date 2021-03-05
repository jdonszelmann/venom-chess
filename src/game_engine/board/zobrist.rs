use crate::game_engine::board::Board;
use crate::game_engine::chess_move::{Move, Location};
use crate::game_engine::piece::Piece;
use crate::game_engine::color::Color;
use std::hash::{Hash, Hasher};
use rand::rngs::StdRng;
use rand::{SeedableRng, RngCore, Rng};
use lazy_static::lazy_static;

pub struct ZobristKeys {
    pieces: [u64; 12],
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

        let mut pieces = [0; 12];
        let mut castling = [0; 4];
        let mut en_passant = [0; 8];

        let color = rng.gen();

        rng.fill(&mut pieces);
        rng.fill(&mut castling);
        rng.fill(&mut en_passant);

        Self {
            pieces,
            color,
            castling,
            en_passant,
        }
    }

    pub fn move_piece(&self, hash: u64, piece_type: Piece, from: Location, replaces: Piece, to: Location) -> u64 {


        0
    }

    pub fn update_castling(&self, hash: u64, previous_rights: [bool; 4], rights: [bool; 4]) -> u64 {


        0
    }

    pub fn update_en_passant(&self, hash: u64, previous_rights: i8, rights: i8) -> u64 {


        0
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

impl<B> ZobristBoard<B> {
    pub fn new(inner: B) -> Self {
        Self {
            inner,
            hash: 0
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

    fn transition_with_move_func(&self, m: Move, mut func: impl FnMut(Piece, Location, Piece, Location)) -> Self {
        let mut hash = self.hash;

        let inner = self.inner.transition_with_move_func(m, |p, f, r, t| {
            hash = ZOBRIST_KEYS.move_piece(hash, p, f, r, t);
            func(p, f, r, t);
        });

        ZOBRIST_KEYS.switch_color(hash);

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

    fn current_player(&self) -> Color {
        self.inner.current_player()
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