use std::fmt;

pub const ORTHOGONAL_DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];
pub const DIAG_DIRECTIONS: [Direction; 4] = [
    Direction::UpLeft,
    Direction::UpRight,
    Direction::DownLeft,
    Direction::DownRight,
];

pub const ALL_DIRECTIONS: [Direction; 8] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
    Direction::UpLeft,
    Direction::UpRight,
    Direction::DownLeft,
    Direction::DownRight,
];

/* When scanning for sequences in a grid, one may want to scan just right/down
 * orthogonally and just down-left/down-right diagonally to avoid getting
 * duplicates from scanning the same sequence from above and below
 */
pub const ORTHOGONAL_DOWN_RIGHT_DIRECTIONS: [Direction; 2] = [Direction::Down, Direction::Right];
pub const DIAG_DOWN_DIRECTIONS: [Direction; 2] = [Direction::DownLeft, Direction::DownLeft];
pub const ALL_DOWN_DIRECTION: [Direction; 4] = [
    Direction::Down,
    Direction::Right,
    Direction::DownLeft,
    Direction::DownRight,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DirectionType {
    Orthogonal,
    Diagonal,
    All,
}

impl Direction {
    pub fn from_offset(offsets: (i32, i32)) -> Option<Self> {
        let (row, col) = offsets;
        let out = match row.cmp(&0) {
            std::cmp::Ordering::Less => match col.cmp(&0) {
                std::cmp::Ordering::Less => Direction::UpLeft,
                std::cmp::Ordering::Equal => Direction::Up,
                std::cmp::Ordering::Greater => Direction::UpRight,
            },
            std::cmp::Ordering::Equal => match col.cmp(&0) {
                std::cmp::Ordering::Less => Direction::Left,
                std::cmp::Ordering::Equal => return None,
                std::cmp::Ordering::Greater => Direction::Right,
            },
            std::cmp::Ordering::Greater => match col.cmp(&0) {
                std::cmp::Ordering::Less => Direction::DownLeft,
                std::cmp::Ordering::Equal => Direction::Down,
                std::cmp::Ordering::Greater => Direction::DownRight,
            },
        };
        Some(out)
    }

    pub fn to_offset(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::UpLeft => (-1, -1),
            Direction::UpRight => (-1, 1),
            Direction::DownLeft => (1, -1),
            Direction::DownRight => (1, 1),
        }
    }

    pub fn left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
            Direction::UpLeft => Direction::DownLeft,
            Direction::DownLeft => Direction::DownRight,
            Direction::DownRight => Direction::UpRight,
            Direction::UpRight => Direction::UpLeft,
        }
    }

    pub fn right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::UpLeft => Direction::UpRight,
            Direction::UpRight => Direction::DownRight,
            Direction::DownRight => Direction::DownLeft,
            Direction::DownLeft => Direction::UpLeft,
        }
    }

    pub fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::UpLeft => Self::DownRight,
            Self::UpRight => Self::DownLeft,
            Self::DownLeft => Self::UpRight,
            Self::DownRight => Self::UpLeft,
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl DirectionType {
    pub fn list_directions(&self) -> std::slice::Iter<'_, Direction> {
        match self {
            DirectionType::Orthogonal => ORTHOGONAL_DIRECTIONS.iter(),
            DirectionType::Diagonal => DIAG_DIRECTIONS.iter(),
            DirectionType::All => ALL_DIRECTIONS.iter(),
        }
    }
}
