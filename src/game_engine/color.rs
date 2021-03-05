#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Color {
    Black,
    White,
    EmptyColor,
}

impl Color {
    pub fn other(&self) -> Self {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black,
            Self::EmptyColor => Self::EmptyColor,
        }
    }
}