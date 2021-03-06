use crate::game_engine::board::Board;
use crate::game_engine::chess_move::{Move, Location};
use crate::game_engine::piece::Piece;
use crate::game_engine::color::Color;
use std::fmt;

const TABLE_PAWN: [[i32; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0, ],
    [50, 50, 50, 50, 50, 50, 50, 50, ],
    [10, 10, 20, 30, 30, 20, 10, 10, ],
    [5, 5, 10, 25, 25, 10, 5, 5, ],
    [0, 0, 0, 20, 20, 0, 0, 0, ],
    [5, -5, -10, 0, 0, -10, -5, 5, ],
    [5, 10, 10, -20, -20, 10, 10, 5, ],
    [0, 0, 0, 0, 0, 0, 0, 0, ],
];

const TABLE_KNIGHT: [[i32; 8]; 8] = [
    [-50, -40, -30, -30, -30, -30, -40, -50, ],
    [-40, -20, 0, 0, 0, 0, -20, -40, ],
    [-30, 0, 10, 15, 15, 10, 0, -30, ],
    [-30, 5, 15, 20, 20, 15, 5, -30, ],
    [-30, 0, 15, 20, 20, 15, 0, -30, ],
    [-30, 5, 10, 15, 15, 10, 5, -30, ],
    [-40, -20, 0, 5, 5, 0, -20, -40, ],
    [-50, -40, -30, -30, -30, -30, -40, -50, ],
];

const TABLE_BISHOP: [[i32; 8]; 8] = [
    [-20, -10, -10, -10, -10, -10, -10, -20, ],
    [-10, 0, 0, 0, 0, 0, 0, -10, ],
    [-10, 0, 5, 10, 10, 5, 0, -10, ],
    [-10, 5, 5, 10, 10, 5, 5, -10, ],
    [-10, 0, 10, 10, 10, 10, 0, -10, ],
    [-10, 10, 10, 10, 10, 10, 10, -10, ],
    [-10, 5, 0, 0, 0, 0, 5, -10, ],
    [-20, -10, -10, -10, -10, -10, -10, -20, ],
];

const TABLE_ROOK: [[i32; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0, ],
    [5, 10, 10, 10, 10, 10, 10, 5, ],
    [-5, 0, 0, 0, 0, 0, 0, -5, ],
    [-5, 0, 0, 0, 0, 0, 0, -5, ],
    [-5, 0, 0, 0, 0, 0, 0, -5, ],
    [-5, 0, 0, 0, 0, 0, 0, -5, ],
    [-5, 0, 0, 0, 0, 0, 0, -5, ],
    [0, 0, 0, 5, 5, 0, 0, 0],
];

const TABLE_QUEEN: [[i32; 8]; 8] = [
    [-20, -10, -10, -5, -5, -10, -10, -20, ],
    [-10, 0, 0, 0, 0, 0, 0, -10, ],
    [-10, 0, 5, 5, 5, 5, 0, -10, ],
    [-5, 0, 5, 5, 5, 5, 0, -5, ],
    [0, 0, 5, 5, 5, 5, 0, -5, ],
    [-10, 5, 5, 5, 5, 5, 0, -10, ],
    [-10, 0, 5, 0, 0, 0, 0, -10, ],
    [-20, -10, -10, -5, -5, -10, -10, -20],
];

const TABLE_KING_MIDDLE: [[i32; 8]; 8] = [
    [-30, -40, -40, -50, -50, -40, -40, -30, ],
    [-30, -40, -40, -50, -50, -40, -40, -30, ],
    [-30, -40, -40, -50, -50, -40, -40, -30, ],
    [-30, -40, -40, -50, -50, -40, -40, -30, ],
    [-20, -30, -30, -40, -40, -30, -30, -20, ],
    [-10, -20, -20, -20, -20, -20, -20, -10, ],
    [20, 20, 0, 0, 0, 0, 20, 20, ],
    [20, 30, 10, 0, 0, 10, 30, 20],
];

pub fn pos_score(p: Piece, l: Location) -> i32 {
    if p.color() == Color::EmptyColor {
        return 0;
    }
    let x = if p.color() == Color::White {
        l.x
    } else {
        7 - l.x
    };
    let y = if p.color() == Color::White {
        l.y
    } else {
        7 - l.y
    };

    let mul = if p.color() == Color::White {
        1
    } else {
        -1
    };

    if p.is_pawn() {
        return TABLE_PAWN[y as usize][x as usize] * mul;
    }
    if p.is_knight() {
        return TABLE_KNIGHT[y as usize][x as usize] * mul;
    }
    if p.is_bishop() {
        return TABLE_BISHOP[y as usize][x as usize] * mul;
    }
    if p.is_rook() {
        return TABLE_ROOK[y as usize][x as usize] * mul;
    }
    if p.is_queen() {
        return TABLE_QUEEN[y as usize][x as usize] * mul;
    }
    if p.is_king() {
        return TABLE_KING_MIDDLE[y as usize][x as usize] * mul;
    }
    0
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct PSTBoard<B> {
    inner: B,
    pub heuristic_value: i32,
}

impl<B: fmt::Display> fmt::Display for PSTBoard<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl<B: Board> PSTBoard<B> {
    pub fn new(inner: B) -> Self {
        Self {
            inner,
            heuristic_value: 0,
        }
    }
}

impl<B> Board for PSTBoard<B> where B: Board {
    #[inline]
    fn moves(&self, location: impl Into<Location>) -> Vec<Move> {
        self.inner.moves(location)
    }

    #[inline]
    fn all_moves(&self) -> Vec<Move> {
        self.inner.all_moves()
    }

    fn transition_with_move_func(&self, m: Move, mut func: impl FnMut(Piece, Location, Location, Piece)) -> Self {
        let mut hv = self.heuristic_value;


        let inner = self.inner.transition_with_move_func(m, |p, f, t, r| {
            hv += self.piece_at(f).material_worth();
            hv -= p.material_worth();
            hv += r.material_worth();

            hv -= pos_score(self.piece_at(f), f);
            hv += pos_score(p, t);
            hv -= pos_score(r, t);

            func(p, f, t, r);
        });


        Self {
            inner,
            heuristic_value: hv,
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
        self.heuristic_value
    }

    #[inline]
    fn piece_at(&self, l: impl Into<Location>) -> Piece {
        self.inner.piece_at(l)
    }

    #[inline]
    fn piece_at_mut(&mut self, l: impl Into<Location>) -> &mut Piece {
        self.inner.piece_at_mut(l)
    }

    fn hash(&self) -> u64 {
        self.inner.hash()
    }
}