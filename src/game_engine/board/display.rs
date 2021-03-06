use crate::game_engine::board::Board;
use crate::game_engine::chess_move::{Move, Location};
use crate::game_engine::piece::Piece;
use crate::game_engine::color::Color;
use std::fmt;


#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct DisplayableBoard<B> {
    pub inner: B,
    highlighted: Vec<Location>,
}


impl<B: Board> fmt::Display for DisplayableBoard<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..8 {
            write!(f, "{} ", y)?;

            for x in 0..8 {

                if (x + y) % 2 == 0{
                    write!(f, "\x1b[100m")?;
                } else {
                    write!(f, "\x1b[45m")?;
                }

                if self.highlighted.contains(&(x, y).into()) {
                    write!(f, "\x1b[103m")?;
                }

                write!(f, "{}", self.piece_at((x, y)))?;
                write!(f, "\x1b[0m")?;

            }
            writeln!(f)?;
        }

        writeln!(f, "   0  1  2  3  4  5  6  7 ")?;
        if self.current_player() == Color::White {
            writeln!(f, "current player: White")?;
        } else {
            writeln!(f, "current player: Black")?;
        }

        Ok(())
    }
}

impl<B: Board> DisplayableBoard<B> {
    pub fn new(inner: B) -> Self {
        Self {
            inner,
            highlighted: Vec::new(),
        }
    }


    pub fn highlight(&mut self, locations: Vec<Location>) {
        self.highlighted = locations;
    }
}

impl<B> Board for DisplayableBoard<B> where B: Board {
    #[inline]
    fn moves(&self, location: impl Into<Location>) -> Vec<Move> {
        self.inner.moves(location)
    }

    #[inline]
    fn all_moves(&self) -> Vec<Move> {
        self.inner.all_moves()
    }

    #[inline]
    fn transition_with_move_func(&self, m: Move, func: impl FnMut(Piece, Location, Location, Piece)) -> Self {
        Self {
            inner: self.inner.transition_with_move_func(m, func),
            highlighted: self.highlighted.clone()
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

    #[inline]
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

    fn hash(&self) -> u64 {
        self.inner.hash()
    }
}
