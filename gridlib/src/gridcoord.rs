pub use crate::direction::Direction;

use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridCoordinate {
    pub x: usize,
    pub y: usize,
}

impl GridCoordinate {
    pub fn new(x: usize, y: usize) -> GridCoordinate {
        return GridCoordinate { x: x, y: y };
    }
}

impl Display for GridCoordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "({}, {})", self.x, self.y);
    }
}

impl std::ops::Add for GridCoordinate {
    type Output = GridCoordinate;

    fn add(self, other: GridCoordinate) -> GridCoordinate {
        return GridCoordinate {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Ord for GridCoordinate {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.y.cmp(&other.y).then_with(|| self.x.cmp(&other.x));
    }
}

impl PartialOrd for GridCoordinate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridCoordinateInf<T: Clone + Copy + Add<Output = T> + From<i32> + Ord> {
    pub x: T,
    pub y: T,
}

impl<T: Clone + Copy + Add<Output = T> + From<i32> + Ord> std::ops::Add for GridCoordinateInf<T> {
    type Output = GridCoordinateInf<T>;

    fn add(self, other: GridCoordinateInf<T>) -> GridCoordinateInf<T> {
        return GridCoordinateInf {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl<T: Clone + Copy + Add<Output = T> + From<i32> + Ord> GridCoordinateInf<T> {
    pub fn new(x: T, y: T) -> GridCoordinateInf<T> {
        return GridCoordinateInf { x: x, y: y };
    }

    pub fn move_dir(&self, direction: Direction) -> GridCoordinateInf<T> {
        let plus_one: T = 1.into();
        let zero: T = 0.into();
        let neg_one: T = (-1).into();
        let north_move = GridCoordinateInf::<T>::new(zero, neg_one);
        let south_move = GridCoordinateInf::<T>::new(zero, plus_one);
        let west_move = GridCoordinateInf::<T>::new(neg_one, zero);
        let east_move = GridCoordinateInf::<T>::new(plus_one, zero);

        return *self
            + match direction {
                Direction::NORTH => north_move,
                Direction::EAST => east_move,
                Direction::SOUTH => south_move,
                Direction::WEST => west_move,
                Direction::NORTHEAST => north_move + east_move,
                Direction::NORTHWEST => north_move + west_move,
                Direction::SOUTHEAST => south_move + east_move,
                Direction::SOUTHWEST => south_move + west_move,
            };
    }
}
impl<T: Clone + Copy + Add<Output = T> + From<i32> + std::ops::Mul<Output = T> + Ord>
    GridCoordinateInf<T>
{
    pub fn move_dir_dist(&self, direction: Direction, distance: T) -> GridCoordinateInf<T> {
        let plus_one: T = 1.into();
        let zero: T = 0.into();
        let neg_one: T = (-1).into();
        let north_move = GridCoordinateInf::<T>::new(zero, neg_one * distance);
        let south_move = GridCoordinateInf::<T>::new(zero, plus_one * distance);
        let west_move = GridCoordinateInf::<T>::new(neg_one * distance, zero);
        let east_move = GridCoordinateInf::<T>::new(plus_one * distance, zero);

        return *self
            + match direction {
                Direction::NORTH => north_move,
                Direction::EAST => east_move,
                Direction::SOUTH => south_move,
                Direction::WEST => west_move,
                Direction::NORTHEAST => north_move + east_move,
                Direction::NORTHWEST => north_move + west_move,
                Direction::SOUTHEAST => south_move + east_move,
                Direction::SOUTHWEST => south_move + west_move,
            };
    }
}

impl<T: Clone + Copy + Add<Output = T> + From<i32> + Display + Ord> Display
    for GridCoordinateInf<T>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "({}, {})", self.x, self.y);
    }
}

impl<T: Clone + Copy + Add<Output = T> + From<i32> + Ord> Ord for GridCoordinateInf<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // You can customize the ordering logic here.
        // For example, you might prioritize y-coordinate, then x-coordinate:
        self.y.cmp(&other.y).then_with(|| self.x.cmp(&other.x))
    }
}

impl<T: Clone + Copy + Add<Output = T> + From<i32> + Ord> PartialOrd for GridCoordinateInf<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub type GridCoordinateInf64 = GridCoordinateInf<i64>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    #[test]
    fn test_format_coord() {
        let a = GridCoordinate::new(2321, 9875);
        let b = format!("{}", a);
        assert_eq!(b, "(2321, 9875)");

        let c = GridCoordinateInf::new(-3213, -9932);
        let d = format!("{}", c);
        assert_eq!(d, "(-3213, -9932)");
    }

    #[test]
    fn test_add_coords() {
        let a = GridCoordinate::new(2321, 9875);
        let b = GridCoordinate::new(1, 5);
        let expected = GridCoordinate::new(2322, 9880);
        assert_eq!(a + b, expected);
    }

    #[test]
    fn test_order_coords() {
        // Earlier y comes before later y
        // Earlier x comes before later x
        // y trumps x.
        let early_y_late_x = GridCoordinate::new(5000, 0);
        let early_x_late_y = GridCoordinate::new(0, 4000);
        let early_y_early_x = GridCoordinate::new(5, 0);
        let late_x_late_y = GridCoordinate::new(5000, 4000);
        assert_eq!(early_y_late_x.cmp(&early_y_late_x), Ordering::Equal);
        assert_eq!(early_y_late_x.cmp(&early_x_late_y), Ordering::Less);
        assert_eq!(early_x_late_y.cmp(&early_y_late_x), Ordering::Greater);
        let mut list = vec![
            late_x_late_y,
            early_x_late_y,
            early_y_late_x,
            early_y_early_x,
            early_y_late_x,
        ];
        list.sort();
        assert_eq!(
            list,
            vec![
                early_y_early_x,
                early_y_late_x,
                early_y_late_x,
                early_x_late_y,
                late_x_late_y
            ]
        );
    }

    #[test]
    fn test_move_on_infinite_grid() {
        let start = GridCoordinateInf::new(0, 0);
        let mut cur = start.move_dir(Direction::NORTH);
        assert_eq!(cur, GridCoordinateInf::new(0, -1));
        cur = cur.move_dir(Direction::WEST);
        assert_eq!(cur, GridCoordinateInf::new(-1, -1));
        cur = cur.move_dir(Direction::NORTHWEST);
        assert_eq!(cur, GridCoordinateInf::new(-2, -2));
        cur = cur.move_dir(Direction::NORTHEAST);
        assert_eq!(cur, GridCoordinateInf::new(-1, -3));
        cur = cur.move_dir(Direction::EAST);
        assert_eq!(cur, GridCoordinateInf::new(-0, -3));
        cur = cur.move_dir(Direction::SOUTH);
        assert_eq!(cur, GridCoordinateInf::new(0, -2));
        cur = cur.move_dir(Direction::SOUTHEAST);
        assert_eq!(cur, GridCoordinateInf::new(1, -1));
        cur = cur.move_dir(Direction::SOUTHWEST);
        assert_eq!(cur, GridCoordinateInf::new(0, 0));
    }

    #[test]
    fn test_move_on_infinite_grid_distance() {
        let start = GridCoordinateInf::new(0, 0);
        let mut cur = start.move_dir_dist(Direction::NORTH, 1);
        assert_eq!(cur, GridCoordinateInf::new(0, -1));
        cur = cur.move_dir_dist(Direction::WEST, 1);
        assert_eq!(cur, GridCoordinateInf::new(-1, -1));
        cur = cur.move_dir_dist(Direction::NORTHWEST, 1);
        assert_eq!(cur, GridCoordinateInf::new(-2, -2));
        cur = cur.move_dir_dist(Direction::NORTHEAST, 1);
        assert_eq!(cur, GridCoordinateInf::new(-1, -3));
        cur = cur.move_dir_dist(Direction::EAST, 1);
        assert_eq!(cur, GridCoordinateInf::new(-0, -3));
        cur = cur.move_dir_dist(Direction::SOUTH, 1);
        assert_eq!(cur, GridCoordinateInf::new(0, -2));
        cur = cur.move_dir_dist(Direction::SOUTHEAST, 1);
        assert_eq!(cur, GridCoordinateInf::new(1, -1));
        cur = cur.move_dir_dist(Direction::SOUTHWEST, 1);
        assert_eq!(cur, GridCoordinateInf::new(0, 0));
    }

    #[test]
    fn test_move_on_infinite_grid64() {
        let start = GridCoordinateInf64::new(0, 0);
        let mut cur = start.move_dir(Direction::NORTH);
        assert_eq!(cur, GridCoordinateInf64::new(0, -1));
        cur = cur.move_dir(Direction::WEST);
        assert_eq!(cur, GridCoordinateInf64::new(-1, -1));
        cur = cur.move_dir(Direction::NORTHWEST);
        assert_eq!(cur, GridCoordinateInf64::new(-2, -2));
        cur = cur.move_dir(Direction::NORTHEAST);
        assert_eq!(cur, GridCoordinateInf64::new(-1, -3));
        cur = cur.move_dir(Direction::EAST);
        assert_eq!(cur, GridCoordinateInf64::new(-0, -3));
        cur = cur.move_dir(Direction::SOUTH);
        assert_eq!(cur, GridCoordinateInf64::new(0, -2));
        cur = cur.move_dir(Direction::SOUTHEAST);
        assert_eq!(cur, GridCoordinateInf64::new(1, -1));
        cur = cur.move_dir(Direction::SOUTHWEST);
        assert_eq!(cur, GridCoordinateInf64::new(0, 0));
    }
}
