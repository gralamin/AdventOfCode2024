use crate::direction::Direction;
use crate::gridcoord::GridCoordinate;

use std::clone::Clone;

#[derive(Debug)]
pub struct Grid<T: Copy> {
    /* Variable sized Grid.
     *
     * width * height = grid_numbers.len()
     * index by: x + (y * width)
     * essentially top left corner is 0,0, right and down increases.
     */
    width: usize,
    height: usize,
    values: Vec<T>,
}

impl<T: Copy> Grid<T> {
    pub fn new(width: usize, height: usize, values: Vec<T>) -> Grid<T> {
        assert_eq!(width * height, values.len());
        return Grid {
            width: width,
            height: height,
            values: values,
        };
    }

    pub fn get_width(&self) -> usize {
        return self.width;
    }

    pub fn get_height(&self) -> usize {
        return self.height;
    }

    pub fn coord_iter(&self) -> GridIter {
        return GridIter {
            cur_x: 0,
            cur_y: 0,
            max_x: self.width,
            max_y: self.height,
            first: true,
        };
    }

    pub fn data_copy(&self) -> Vec<T>
    where
        T: Clone,
    {
        return self.values.clone();
    }

    fn coord_direction_iterator(
        &self,
        pos: GridCoordinate,
        direction_iter: std::slice::Iter<Direction>,
    ) -> Vec<(GridCoordinate, Direction)> {
        let mut result: Vec<(GridCoordinate, Direction)> = Vec::new();
        for &direction in direction_iter {
            let coord = self.get_coordinate_by_direction(pos, direction);
            if let Some(cur_pos) = coord {
                result.push((cur_pos, direction));
            }
        }
        return result;
    }
}

impl<T: Clone + Copy> Clone for Grid<T> {
    fn clone(&self) -> Self {
        return Self::new(self.width, self.height, self.values.clone());
    }
}

impl<T: PartialEq + Copy> PartialEq for Grid<T> {
    fn eq(&self, other: &Self) -> bool {
        self.height == other.height && self.width == other.width && self.values == other.data_copy()
    }
}
impl<T: Eq + Copy> Eq for Grid<T> {}

pub struct GridIter {
    cur_x: usize,
    cur_y: usize,
    max_x: usize,
    max_y: usize,
    first: bool,
}

impl Iterator for GridIter {
    type Item = GridCoordinate;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            return Some(GridCoordinate::new(self.cur_x, self.cur_y));
        }
        self.cur_x += 1;
        if self.cur_x >= self.max_x {
            self.cur_x = self.cur_x % self.max_x;
            self.cur_y += 1;
        }
        if self.cur_y >= self.max_y {
            return None;
        } else {
            return Some(GridCoordinate::new(self.cur_x, self.cur_y));
        }
    }
}

pub trait GridTraversable {
    type Item;

    fn get_value(&self, pos: GridCoordinate) -> Option<Self::Item>;
    fn set_value(&mut self, pos: GridCoordinate, value: Self::Item);
    fn get_coordinate_by_direction(
        &self,
        pos: GridCoordinate,
        direction: Direction,
    ) -> Option<GridCoordinate>;
    fn get_adjacent_coordinates(&self, pos: GridCoordinate) -> Vec<GridCoordinate>;
    fn get_adjacent_coordinates_and_direction(
        &self,
        pos: GridCoordinate,
    ) -> Vec<(GridCoordinate, Direction)>;
    fn get_diag_adjacent_coordinates(&self, pos: GridCoordinate) -> Vec<GridCoordinate>;
    fn get_diag_adjacent_coordinates_and_direction(
        &self,
        pos: GridCoordinate,
    ) -> Vec<(GridCoordinate, Direction)>;
    fn get_all_adjacent_coordinates(&self, pos: GridCoordinate) -> Vec<GridCoordinate>;
    fn get_all_adjacent_coordinates_and_direction(
        &self,
        pos: GridCoordinate,
    ) -> Vec<(GridCoordinate, Direction)>;
}

impl<T: Copy> GridTraversable for Grid<T> {
    type Item = T;

    fn get_value(&self, pos: GridCoordinate) -> Option<Self::Item> {
        if pos.y >= self.height || pos.x >= self.width {
            // y cannot exceed height, x cannot exceed width
            return None;
        }
        let pos: usize = pos.x + pos.y * self.width;
        return Some(*(self.values.iter().nth(pos)?));
    }

    fn set_value(&mut self, pos: GridCoordinate, value: Self::Item) {
        if pos.y >= self.height || pos.x >= self.width {
            // y cannot exceed height, x cannot exceed width
            return;
        }
        let pos: usize = pos.x + pos.y * self.width;
        self.values[pos] = value;
    }

    fn get_coordinate_by_direction(
        &self,
        pos: GridCoordinate,
        direction: Direction,
    ) -> Option<GridCoordinate> {
        let mut possible_y: Option<usize> = Some(pos.y);
        let mut possible_x: Option<usize> = Some(pos.x);
        match direction {
            Direction::NORTH => possible_y = pos.y.checked_sub(1),
            Direction::EAST => possible_x = pos.x.checked_add(1),
            Direction::SOUTH => possible_y = pos.y.checked_add(1),
            Direction::WEST => possible_x = pos.x.checked_sub(1),
            Direction::NORTHEAST => {
                possible_x = pos.x.checked_add(1);
                possible_y = pos.y.checked_sub(1);
            }
            Direction::SOUTHEAST => {
                possible_x = pos.x.checked_add(1);
                possible_y = pos.y.checked_add(1);
            }
            Direction::SOUTHWEST => {
                possible_x = pos.x.checked_sub(1);
                possible_y = pos.y.checked_add(1);
            }
            Direction::NORTHWEST => {
                possible_x = pos.x.checked_sub(1);
                possible_y = pos.y.checked_sub(1);
            }
        }
        if let Some(new_x) = possible_x {
            if let Some(new_y) = possible_y {
                if new_x > self.width - 1 || new_y > self.height - 1 {
                    return None;
                }
                return Some(GridCoordinate::new(new_x, new_y));
            }
        }
        return None;
    }

    fn get_adjacent_coordinates(&self, pos: GridCoordinate) -> Vec<GridCoordinate> {
        return self
            .coord_direction_iterator(pos, Direction::cardinal_iterator())
            .into_iter()
            .map(|x| x.0)
            .collect();
    }

    fn get_adjacent_coordinates_and_direction(
        &self,
        pos: GridCoordinate,
    ) -> Vec<(GridCoordinate, Direction)> {
        return self.coord_direction_iterator(pos, Direction::cardinal_iterator());
    }

    fn get_diag_adjacent_coordinates(&self, pos: GridCoordinate) -> Vec<GridCoordinate> {
        return self
            .coord_direction_iterator(pos, Direction::diagonal_iterator())
            .into_iter()
            .map(|x| x.0)
            .collect();
    }

    fn get_diag_adjacent_coordinates_and_direction(
        &self,
        pos: GridCoordinate,
    ) -> Vec<(GridCoordinate, Direction)> {
        return self.coord_direction_iterator(pos, Direction::diagonal_iterator());
    }

    fn get_all_adjacent_coordinates(&self, pos: GridCoordinate) -> Vec<GridCoordinate> {
        return self
            .coord_direction_iterator(pos, Direction::iterator())
            .into_iter()
            .map(|x| x.0)
            .collect();
    }

    fn get_all_adjacent_coordinates_and_direction(
        &self,
        pos: GridCoordinate,
    ) -> Vec<(GridCoordinate, Direction)> {
        return self.coord_direction_iterator(pos, Direction::iterator());
    }
}

pub trait GridRotation {
    type Item;
    fn rotate_clockwise(&mut self);
}

impl<T: Copy> GridRotation for Grid<T> {
    type Item = T;

    fn rotate_clockwise(&mut self) {
        let data_copy = self.values.clone();
        let n = self.get_height();
        let m = self.get_width();
        let new_height = m;
        let new_width = n;
        for i in 0..n {
            for j in 0..m {
                let old_value = data_copy[i * m + j];
                let col = n - 1 - i;
                self.values[j * new_width + col] = old_value;
            }
        }
        self.width = new_width;
        self.height = new_height;
    }
}

pub trait GridPrintable {
    fn get_character(&self) -> char;
}

impl GridPrintable for char {
    fn get_character(&self) -> char {
        return self.clone();
    }
}

pub trait GridOverlay: GridPrintable {
    fn get_position(&self) -> GridCoordinate;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct SimpleGridOverlay {
    character: char,
    position: GridCoordinate,
}

impl GridOverlay for SimpleGridOverlay {
    fn get_position(&self) -> GridCoordinate {
        return self.position;
    }
}

impl GridPrintable for SimpleGridOverlay {
    fn get_character(&self) -> char {
        return self.character;
    }
}

impl SimpleGridOverlay {
    pub fn new(c: char, pos: GridCoordinate) -> SimpleGridOverlay {
        return SimpleGridOverlay {
            character: c,
            position: pos,
        };
    }
}

impl<T: Copy + GridPrintable> Grid<T> {
    pub fn grid_strings(&self) -> Vec<String> {
        let mut values = vec!['X'; self.get_height() * self.get_width()];
        let width = self.get_width();
        for coord in self.coord_iter() {
            let v = self.get_value(coord).unwrap();
            let index = coord.x + coord.y * width;
            values[index] = v.get_character();
        }

        let mut lines = vec![];
        for (x, c) in values.into_iter().enumerate() {
            if x % width == 0 {
                lines.push(vec![]);
            }
            lines.last_mut().unwrap().push(c);
        }
        return lines
            .into_iter()
            .map(|line| line.into_iter().collect::<String>())
            .collect();
    }

    pub fn grid_strings_with_overlay<J, I>(&self, overlay: I) -> Vec<String>
    where
        J: GridOverlay,
        I: IntoIterator<Item = J>,
    {
        let mut values = vec!['X'; self.get_height() * self.get_width()];
        let width = self.get_width();
        for coord in self.coord_iter() {
            let v = self.get_value(coord).unwrap();
            let index = coord.x + coord.y * width;
            values[index] = v.get_character();
        }

        for v in overlay {
            let coord = v.get_position();
            let index = coord.x + coord.y * width;
            values[index] = v.get_character();
        }

        let mut lines = vec![];
        for (x, c) in values.into_iter().enumerate() {
            if x % width == 0 {
                lines.push(vec![]);
            }
            lines.last_mut().unwrap().push(c);
        }
        return lines
            .into_iter()
            .map(|line| line.into_iter().collect::<String>())
            .collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_make_bad_grid() {
        let nums = vec![1, 2];
        let height = 9;
        let width = 23;
        Grid::new(width, height, nums);
    }

    fn produce_grid() -> Grid<i32> {
        let grid_nums = vec![
            2, 1, 9, 9, 9, 4, 3, 2, 1, 0, 3, 9, 8, 7, 8, 9, 4, 9, 2, 1, 9, 8, 5, 6, 7, 8, 9, 8, 9,
            2, 8, 7, 6, 7, 8, 9, 6, 7, 8, 9, 9, 8, 9, 9, 9, 6, 5, 6, 7, 8,
        ];
        let grid: Grid<i32> = Grid::new(10, 5, grid_nums);
        return grid;
    }

    #[test]
    fn test_clone() {
        let grid = produce_grid();
        let clone = grid.clone();
        assert_eq!(grid, clone);
    }

    #[test]
    fn test_get_grid_number() {
        let grid = produce_grid();
        assert_eq!(grid.get_value(GridCoordinate::new(0, 0)), Some(2));
        assert_eq!(grid.get_value(GridCoordinate::new(9, 0)), Some(0));
        assert_eq!(grid.get_value(GridCoordinate::new(0, 4)), Some(9));
        assert_eq!(grid.get_value(GridCoordinate::new(9, 4)), Some(8));
        assert_eq!(grid.get_value(GridCoordinate::new(4, 2)), Some(7));
        assert_eq!(grid.get_value(GridCoordinate::new(5, 2)), Some(8));
    }

    #[test]
    fn test_set_grid_number() {
        let mut grid = produce_grid();
        let coord = GridCoordinate::new(3, 3);
        assert_eq!(grid.get_value(coord), Some(7));
        grid.set_value(coord, 99);
        assert_eq!(grid.get_value(coord), Some(99));
    }

    #[test]
    fn test_set_invalid_grid_number() {
        let mut grid = produce_grid();
        let coord = GridCoordinate::new(300000, 3);
        grid.set_value(coord, 99);
    }

    #[test]
    fn test_get_invalid_grid_number() {
        let grid = produce_grid();
        let coord = GridCoordinate::new(300000, 3);
        assert_eq!(grid.get_value(coord), None);
    }

    #[test]
    fn test_get_adjacent_coordinates() {
        let grid = produce_grid();
        assert_eq!(
            grid.get_adjacent_coordinates(GridCoordinate::new(0, 0)),
            vec![GridCoordinate::new(1, 0), GridCoordinate::new(0, 1)]
        );
        assert_eq!(
            grid.get_adjacent_coordinates(GridCoordinate::new(9, 0)),
            vec![GridCoordinate::new(9, 1), GridCoordinate::new(8, 0)]
        );
        assert_eq!(
            grid.get_adjacent_coordinates(GridCoordinate::new(0, 4)),
            vec![GridCoordinate::new(0, 3), GridCoordinate::new(1, 4)]
        );
        assert_eq!(
            grid.get_adjacent_coordinates(GridCoordinate::new(9, 4)),
            vec![GridCoordinate::new(9, 3), GridCoordinate::new(8, 4)]
        );
    }

    #[test]
    fn test_get_adjacent_coordinates_and_direction() {
        let grid = produce_grid();
        assert_eq!(
            grid.get_adjacent_coordinates_and_direction(GridCoordinate::new(0, 0)),
            vec![
                (GridCoordinate::new(1, 0), Direction::EAST),
                (GridCoordinate::new(0, 1), Direction::SOUTH)
            ]
        );
    }

    #[test]
    fn test_get_diag_adjacent_coordinates() {
        let grid = produce_grid();
        assert_eq!(
            grid.get_diag_adjacent_coordinates(GridCoordinate::new(0, 0)),
            vec![GridCoordinate::new(1, 1)]
        );
        assert_eq!(
            grid.get_diag_adjacent_coordinates(GridCoordinate::new(9, 0)),
            vec![GridCoordinate::new(8, 1)]
        );
        assert_eq!(
            grid.get_diag_adjacent_coordinates(GridCoordinate::new(0, 4)),
            vec![GridCoordinate::new(1, 3)]
        );
        assert_eq!(
            grid.get_diag_adjacent_coordinates(GridCoordinate::new(9, 4)),
            vec![GridCoordinate::new(8, 3)]
        );
    }

    #[test]
    fn test_get_diag_adjacent_coordinates_and_directions() {
        let grid = produce_grid();
        assert_eq!(
            grid.get_diag_adjacent_coordinates_and_direction(GridCoordinate::new(0, 0)),
            vec![(GridCoordinate::new(1, 1), Direction::SOUTHEAST)]
        );
    }

    #[test]
    fn test_all_adjacent_coordinates_and_directions() {
        let grid = produce_grid();
        assert_eq!(
            grid.get_all_adjacent_coordinates_and_direction(GridCoordinate::new(0, 0)),
            vec![
                (GridCoordinate { x: 1, y: 0 }, Direction::EAST),
                (GridCoordinate { x: 1, y: 1 }, Direction::SOUTHEAST),
                (GridCoordinate { x: 0, y: 1 }, Direction::SOUTH)
            ]
        );
    }

    #[test]
    fn test_all_adjacent_coordinates() {
        let grid = produce_grid();
        assert_eq!(
            grid.get_all_adjacent_coordinates(GridCoordinate::new(0, 0)),
            vec![
                GridCoordinate { x: 1, y: 0 },
                GridCoordinate { x: 1, y: 1 },
                GridCoordinate { x: 0, y: 1 }
            ]
        );
    }

    #[test]
    fn test_get_width() {
        let grid = produce_grid();
        assert_eq!(grid.get_width(), 10);
    }

    #[test]
    fn test_get_height() {
        let grid = produce_grid();
        assert_eq!(grid.get_height(), 5);
    }

    #[test]
    fn test_coord_iter() {
        let grid = produce_grid();
        let mut iter = grid.coord_iter();
        if let Some(first_v) = iter.next() {
            assert_eq!(first_v, GridCoordinate::new(0, 0));
        } else {
            panic!("No first value found");
        }

        if let Some(second_v) = iter.next() {
            assert_eq!(second_v, GridCoordinate::new(1, 0));
        } else {
            panic!("No second value found");
        }

        let all: Vec<GridCoordinate> = grid.coord_iter().collect();
        assert_eq!(all.len(), 50);
    }

    #[test]
    fn test_add_coords() {
        let a = GridCoordinate::new(3, 5);
        let b = GridCoordinate::new(7, 11);
        let expected = GridCoordinate::new(10, 16);
        assert_eq!(a + b, expected);
    }

    #[test]
    fn test_rotate_grid() {
        let mut grid = produce_grid();
        grid.rotate_clockwise();
        let data = grid.data_copy();
        assert_eq!(
            data,
            vec![
                9, 8, 9, 3, 2, 8, 7, 8, 9, 1, 9, 6, 5, 8, 9, 9, 7, 6, 7, 9, 9, 8, 7, 8, 9, 6, 9, 8,
                9, 4, 5, 6, 9, 4, 3, 6, 7, 8, 9, 2, 7, 8, 9, 2, 1, 8, 9, 2, 1, 0
            ]
        );
    }

    #[test]
    fn test_print_grid() {
        #[derive(Copy, Clone, Debug)]
        struct TestValue {
            c: char,
        }
        impl GridPrintable for TestValue {
            fn get_character(&self) -> char {
                return self.c;
            }
        }

        let grid_values = vec![
            TestValue { c: '+' },
            TestValue { c: '-' },
            TestValue { c: '-' },
            TestValue { c: '-' },
            TestValue { c: '+' },
            TestValue { c: '|' },
            TestValue { c: '.' },
            TestValue { c: '.' },
            TestValue { c: '.' },
            TestValue { c: '|' },
            TestValue { c: '|' },
            TestValue { c: '.' },
            TestValue { c: 'C' },
            TestValue { c: '.' },
            TestValue { c: '|' },
            TestValue { c: '|' },
            TestValue { c: '.' },
            TestValue { c: '.' },
            TestValue { c: '.' },
            TestValue { c: '|' },
            TestValue { c: '+' },
            TestValue { c: '-' },
            TestValue { c: '-' },
            TestValue { c: '-' },
            TestValue { c: '+' },
        ];
        let grid = Grid::new(5, 5, grid_values);
        let strings = grid.grid_strings().join("\n");
        assert_eq!(strings, "+---+\n|...|\n|.C.|\n|...|\n+---+");
    }

    #[test]
    fn test_print_grid_overlay() {
        #[derive(Copy, Clone, Debug)]
        struct TestValue {
            c: char,
        }

        impl GridPrintable for TestValue {
            fn get_character(&self) -> char {
                return self.c;
            }
        }

        #[derive(Copy, Clone, Debug)]
        struct Overlay {
            coord: GridCoordinate,
        }

        impl GridPrintable for Overlay {
            fn get_character(&self) -> char {
                return '@';
            }
        }

        impl GridOverlay for Overlay {
            fn get_position(&self) -> GridCoordinate {
                return self.coord;
            }
        }

        let grid_values = vec![
            TestValue { c: '+' },
            TestValue { c: '-' },
            TestValue { c: '-' },
            TestValue { c: '-' },
            TestValue { c: '+' },
            TestValue { c: '|' },
            TestValue { c: '.' },
            TestValue { c: '.' },
            TestValue { c: '.' },
            TestValue { c: '|' },
            TestValue { c: '|' },
            TestValue { c: '.' },
            TestValue { c: 'C' },
            TestValue { c: '.' },
            TestValue { c: '|' },
            TestValue { c: '|' },
            TestValue { c: '.' },
            TestValue { c: '.' },
            TestValue { c: '.' },
            TestValue { c: '|' },
            TestValue { c: '+' },
            TestValue { c: '-' },
            TestValue { c: '-' },
            TestValue { c: '-' },
            TestValue { c: '+' },
        ];
        let grid = Grid::new(5, 5, grid_values);
        let overlay = vec![
            Overlay {
                coord: GridCoordinate::new(1, 1),
            },
            Overlay {
                coord: GridCoordinate::new(1, 2),
            },
            Overlay {
                coord: GridCoordinate::new(2, 2),
            },
            Overlay {
                coord: GridCoordinate::new(0, 1),
            },
            Overlay {
                coord: GridCoordinate::new(3, 4),
            },
        ];

        let strings = grid.grid_strings_with_overlay(overlay).join("\n");
        assert_eq!(strings, "+---+\n@@..|\n|@@.|\n|...|\n+--@+");
    }
}
