
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub struct Location {
    pub x: u8,
    pub y: u8,
}

impl Location {
    pub fn new(x: u8, y: u8) -> Self {
        assert!(x >= 0 && x < 8);
        assert!(y >= 0 && y < 8);

        Self {
            x, y
        }
    }
}

impl From<(u8, u8)> for Location {
    fn from((x, y): (u8, u8)) -> Self {
        Self::new(x, y)
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub struct Move {
    pub from: Location,
    pub to: Location,
}

impl Move {
    pub fn new(from: Location, to: Location) -> Self {
        Self {
            from,
            to
        }
    }
}

impl From<(Location, Location)> for Move {
    fn from((from, to): (Location, Location)) -> Self {
        Self {
            from, to
        }
    }
}

impl From<((u8, u8), (u8, u8))> for Move {
    fn from((from, to): ((u8, u8), (u8, u8))) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
        }
    }
}