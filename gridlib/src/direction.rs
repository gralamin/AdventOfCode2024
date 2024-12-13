use std::fmt::{Display, Formatter};
use std::slice::Iter;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
    NORTHEAST,
    SOUTHEAST,
    SOUTHWEST,
    NORTHWEST,
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Direction::NORTH => "NORTH",
            Direction::EAST => "EAST",
            Direction::SOUTH => "SOUTH",
            Direction::WEST => "WEST",
            Direction::NORTHEAST => "NORTHEAST",
            Direction::SOUTHEAST => "SOUTHEAST",
            Direction::SOUTHWEST => "SOUTHWEST",
            Direction::NORTHWEST => "NORTHWEST",
        };
        return write!(f, "{}", s);
    }
}

impl Direction {
    pub fn cardinal_iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [
            Direction::NORTH,
            Direction::EAST,
            Direction::SOUTH,
            Direction::WEST,
        ];
        return DIRECTIONS.iter();
    }

    pub fn diagonal_iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [
            Direction::NORTHEAST,
            Direction::SOUTHEAST,
            Direction::SOUTHWEST,
            Direction::NORTHWEST,
        ];
        DIRECTIONS.iter()
    }

    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 8] = [
            Direction::NORTH,
            Direction::NORTHEAST,
            Direction::EAST,
            Direction::SOUTHEAST,
            Direction::SOUTH,
            Direction::SOUTHWEST,
            Direction::WEST,
            Direction::NORTHWEST,
        ];
        DIRECTIONS.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_direction() {
        assert_eq!(format!("{}", Direction::NORTH), "NORTH");
        assert_eq!(format!("{}", Direction::EAST), "EAST");
        assert_eq!(format!("{}", Direction::SOUTH), "SOUTH");
        assert_eq!(format!("{}", Direction::WEST), "WEST");
        assert_eq!(format!("{}", Direction::NORTHEAST), "NORTHEAST");
        assert_eq!(format!("{}", Direction::NORTHWEST), "NORTHWEST");
        assert_eq!(format!("{}", Direction::SOUTHEAST), "SOUTHEAST");
        assert_eq!(format!("{}", Direction::SOUTHWEST), "SOUTHWEST");
    }
}
