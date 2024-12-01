use std::fmt::{Display, Formatter};

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
