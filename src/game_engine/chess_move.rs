
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub struct Location {
    pub x: i8,
    pub y: i8,
}

impl Location {
    pub fn new(x: i8, y: i8) -> Self {
        assert!(x >= 0 && x < 8);
        assert!(y >= 0 && y < 8);

        Self {
            x, y
        }
    }
}

impl From<(i8, i8)> for Location {
    fn from((x, y): (i8, i8)) -> Self {
        Self::new(x, y)
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub struct Move {
    pub from: Location,
    pub to: Location,
    pub extra: u8,
}

impl Move {
    pub fn new(from: Location, to: Location, extra: u8) -> Self {
        Self {
            from,
            to,
            extra,
        }
    }
}

impl From<(Location, Location)> for Move {
    fn from((from, to): (Location, Location)) -> Self {
        Self {
            from, to, extra:0,
        }
    }
}

impl From<(Location, Location, u8)> for Move {
    fn from((from, to, extra): (Location, Location, u8)) -> Self {
        Self {
            from, to, extra,
        }
    }
}

impl From<((i8, i8), (i8, i8))> for Move {
    fn from((from, to): ((i8, i8), (i8, i8))) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
            extra: 0
        }
    }
}

impl From<((i8, i8), (i8, i8), u8)> for Move {
    fn from((from, to, extra): ((i8, i8), (i8, i8), u8)) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
            extra,
        }
    }
}