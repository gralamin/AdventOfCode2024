extern crate filelib;

pub use filelib::load_no_blanks;
use gridlib::GridTraversable;
use gridlib::{Grid, GridCoordinate};
use log::info;
use std::collections::HashSet;

type Map = Grid<Option<Antenna>>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Antenna {
    position: GridCoordinate,
    frequency: char,
}

impl Antenna {
    fn new(position: GridCoordinate, frequency: char) -> Antenna {
        return Antenna {
            position: position,
            frequency: frequency,
        };
    }

    fn get_antinodes(&self, other: &Antenna, grid: &Map) -> Vec<GridCoordinate> {
        info!("Trying to find Antinode for {:?}, {:?}", self, other);
        let mut result: Vec<GridCoordinate> = vec![];
        if other.frequency != self.frequency {
            return result;
        }
        // Antinodes are twice the distance away as other is.
        let x_diff;
        let y_diff;
        let antinode_1_x;
        let antinode_1_y;
        let antinode_2_x;
        let antinode_2_y;
        if self.position.x > other.position.x {
            x_diff = self.position.x - other.position.x;
            antinode_1_x = other.position.x.checked_sub(x_diff);
            antinode_2_x = self.position.x.checked_add(x_diff);
        } else {
            x_diff = other.position.x - self.position.x;
            antinode_1_x = other.position.x.checked_add(x_diff);
            antinode_2_x = self.position.x.checked_sub(x_diff);
        }
        if self.position.y > other.position.y {
            y_diff = self.position.y - other.position.y;
            antinode_1_y = other.position.y.checked_sub(y_diff);
            antinode_2_y = self.position.y.checked_add(y_diff);
        } else {
            y_diff = other.position.y - self.position.y;
            antinode_1_y = other.position.y.checked_add(y_diff);
            antinode_2_y = self.position.y.checked_sub(y_diff);
        }
        if antinode_1_x.is_some() && antinode_1_y.is_some() {
            // check if on grid
            let x = antinode_1_x.unwrap();
            let y = antinode_1_y.unwrap();
            if x < grid.get_width() && y < grid.get_height() {
                result.push(GridCoordinate::new(x, y));
            }
        }
        if antinode_2_x.is_some() && antinode_2_y.is_some() {
            // check if on grid
            let x = antinode_2_x.unwrap();
            let y = antinode_2_y.unwrap();
            if x < grid.get_width() && y < grid.get_height() {
                result.push(GridCoordinate::new(x, y));
            }
        }
        info!("Antinodes found {:?}", result);
        return result;
    }

    fn get_continual_antinodes(&self, other: &Antenna, grid: &Map) -> Vec<GridCoordinate> {
        info!("Trying to find Antinode for {:?}, {:?}", self, other);
        let mut result: Vec<GridCoordinate> = vec![];
        if other.frequency != self.frequency {
            return result;
        }
        result.push(self.position);
        result.push(other.position);
        // Antinodes are twice the distance away as other is.
        let x_diff;
        let y_diff;
        let mut antinode_1_x;
        let mut antinode_1_y;
        let mut antinode_2_x;
        let mut antinode_2_y;
        let self_add_x: bool = self.position.x > other.position.x;
        let self_add_y: bool = self.position.y > other.position.y;
        if self_add_x {
            x_diff = self.position.x - other.position.x;
            antinode_1_x = other.position.x.checked_sub(x_diff);
            antinode_2_x = self.position.x.checked_add(x_diff);
        } else {
            x_diff = other.position.x - self.position.x;
            antinode_1_x = other.position.x.checked_add(x_diff);
            antinode_2_x = self.position.x.checked_sub(x_diff);
        }
        if self_add_y {
            y_diff = self.position.y - other.position.y;
            antinode_1_y = other.position.y.checked_sub(y_diff);
            antinode_2_y = self.position.y.checked_add(y_diff);
        } else {
            y_diff = other.position.y - self.position.y;
            antinode_1_y = other.position.y.checked_add(y_diff);
            antinode_2_y = self.position.y.checked_sub(y_diff);
        }
        while antinode_1_x.is_some() && antinode_1_y.is_some() {
            // check if on grid
            let x = antinode_1_x.unwrap();
            let y = antinode_1_y.unwrap();
            
            if x < grid.get_width() && y < grid.get_height() {
                result.push(GridCoordinate::new(x, y));
            } else {
                // Off grid
                break;
            }
            if self_add_x {
                // this is the other x, so subtrack
                antinode_1_x = x.checked_sub(x_diff);
            } else {
                antinode_1_x = x.checked_add(x_diff);
            }
            if self_add_y {
                antinode_1_y = y.checked_sub(y_diff);
            } else {
                antinode_1_y = y.checked_add(y_diff);
            }
        }
        while antinode_2_x.is_some() && antinode_2_y.is_some() {
            // check if on grid
            let x = antinode_2_x.unwrap();
            let y = antinode_2_y.unwrap();
            if x < grid.get_width() && y < grid.get_height() {
                result.push(GridCoordinate::new(x, y));
            } else {
                // off grid
                break;
            }
            if self_add_x {
                antinode_2_x = x.checked_add(x_diff);
            } else {
                antinode_2_x = x.checked_sub(x_diff);
            }
            if self_add_y {
                antinode_2_y = y.checked_add(y_diff);
            } else {
                antinode_2_y = y.checked_sub(y_diff);
            }
        }
        info!("Antinodes found {:?}", result);
        return result;
    }
}

fn parse_map(string_list: &Vec<String>) -> Map {
    let width = string_list.first().unwrap().len();
    let height = string_list.len();
    let mut values = vec![];
    for (y, line) in string_list.into_iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let v;
            match c {
                'A'..='Z' | 'a'..='z' | '0'..='9' => {
                    v = Some(Antenna::new(GridCoordinate::new(x, y), c));
                }
                '.' => v = None,
                _ => panic!("Unknown char {}", c),
            };
            values.push(v);
        }
    }
    return Map::new(width, height, values);
}

/// Count antinodes
/// ```
/// let vec1: Vec<String> = vec![
///     "............",
///     "........0...",
///     ".....0......",
///     ".......0....",
///     "....0.......",
///     "......A.....",
///     "............",
///     "............",
///     "........A...",
///     ".........A..",
///     "............",
///     "............"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day08::puzzle_a(&vec1), 14);
/// ```
pub fn puzzle_a(string_list: &Vec<String>) -> usize {
    let map = parse_map(string_list);
    let mut antinodes = HashSet::new();
    let mut antennas = vec![];
    for coord in map.coord_iter() {
        let v = map.get_value(coord).unwrap();
        if v.is_none() {
            continue;
        }
        antennas.push(v.unwrap());
    }
    for (i, antenna) in antennas.clone().into_iter().enumerate() {
        for j in i + 1..antennas.len() {
            let cur_nodes: Vec<GridCoordinate> = antenna.get_antinodes(&antennas[j], &map);
            antinodes.extend(cur_nodes);
        }
    }
    return antinodes.len();
}

/// Even more antinodes
/// ```
/// let vec1: Vec<String> = vec![
///     "............",
///     "........0...",
///     ".....0......",
///     ".......0....",
///     "....0.......",
///     "......A.....",
///     "............",
///     "............",
///     "........A...",
///     ".........A..",
///     "............",
///     "............"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day08::puzzle_b(&vec1), 34);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> usize {
    let map = parse_map(string_list);
    let mut antinodes = HashSet::new();
    let mut antennas = vec![];
    for coord in map.coord_iter() {
        let v = map.get_value(coord).unwrap();
        if v.is_none() {
            continue;
        }
        antennas.push(v.unwrap());
    }
    for (i, antenna) in antennas.clone().into_iter().enumerate() {
        for j in i + 1..antennas.len() {
            let cur_nodes: Vec<GridCoordinate> = antenna.get_continual_antinodes(&antennas[j], &map);
            antinodes.extend(cur_nodes);
        }
    }
    return antinodes.len();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_attenna_one() -> Antenna {
        let position = GridCoordinate::new(3, 3);
        return Antenna::new(position, '1');
    }

    fn test_attenna_two() -> Antenna {
        let position = GridCoordinate::new(5, 5);
        return Antenna::new(position, '1');
    }

    fn test_attenna_diff() -> Antenna {
        let position = GridCoordinate::new(4, 4);
        return Antenna::new(position, 'z');
    }

    fn test_attenna_extreme_one() -> Antenna {
        let position = GridCoordinate::new(0, 0);
        return Antenna::new(position, 'e');
    }

    fn test_attenna_extreme_two() -> Antenna {
        let position = GridCoordinate::new(9, 9);
        return Antenna::new(position, 'e');
    }

    fn generate_grid(antennas: Vec<Antenna>) -> Map {
        let width: usize = 10;
        let height: usize = 10;
        let mut values: Vec<Option<Antenna>> = vec![
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ];
        for antenna in antennas {
            values[antenna.position.x + antenna.position.y * width] = Some(antenna.clone());
        }
        return Map::new(width, height, values);
    }

    #[test]
    fn test_no_antinode() {
        let first = test_attenna_one();
        let second = test_attenna_diff();
        let both = vec![first, second];
        let grid = generate_grid(both);
        assert_eq!(first.get_antinodes(&second, &grid), vec![]);
    }

    #[test]
    fn test_antinodes_on_map() {
        let first = test_attenna_one();
        let second = test_attenna_two();
        let both = vec![first, second];
        let grid = generate_grid(both);
        assert_eq!(
            first.get_antinodes(&second, &grid),
            vec![GridCoordinate::new(7, 7), GridCoordinate::new(1, 1)]
        );
    }

    #[test]
    fn test_antennas_off_map() {
        let first = test_attenna_extreme_one();
        let second = test_attenna_extreme_two();
        let both = vec![first, second];
        let grid = generate_grid(both);
        assert_eq!(first.get_antinodes(&second, &grid), vec![]);
    }

    #[test]
    fn test_antennas_debug_one() {
        let first = Antenna::new(GridCoordinate::new(4, 3), '0');
        let second = Antenna::new(GridCoordinate::new(5, 5), '0');
        let both = vec![first, second];
        let grid = generate_grid(both);
        assert_eq!(
            first.get_antinodes(&second, &grid),
            vec![GridCoordinate::new(6, 7), GridCoordinate::new(3, 1)]
        );
    }

    #[test]
    fn test_antennas_debug_two() {
        let first = Antenna::new(GridCoordinate::new(4, 3), '0');
        let second = Antenna::new(GridCoordinate::new(5, 5), '0');
        let third = Antenna::new(GridCoordinate::new(8, 4), '0');
        let all = vec![first, second, third];
        let grid = generate_grid(all);
        assert_eq!(
            first.get_antinodes(&second, &grid),
            vec![GridCoordinate::new(6, 7), GridCoordinate::new(3, 1)]
        );
        assert_eq!(
            first.get_antinodes(&third, &grid),
            vec![GridCoordinate::new(0, 2)]
        );
        assert_eq!(
            second.get_antinodes(&third, &grid),
            vec![GridCoordinate::new(2, 6)]
        );
    }

    #[test]
    fn test_antennas_horizontal() {
        let first = Antenna::new(GridCoordinate::new(2, 0), '0');
        let second = Antenna::new(GridCoordinate::new(3, 0), '0');
        let all = vec![first, second];
        let grid = generate_grid(all);
        assert_eq!(
            first.get_antinodes(&second, &grid),
            vec![GridCoordinate::new(4, 0), GridCoordinate::new(1, 0)]
        );
    }

    #[test]
    fn test_antennas_vertical() {
        let first = Antenna::new(GridCoordinate::new(0, 2), '0');
        let second = Antenna::new(GridCoordinate::new(0, 3), '0');
        let all = vec![first, second];
        let grid = generate_grid(all);
        assert_eq!(
            first.get_antinodes(&second, &grid),
            vec![GridCoordinate::new(0, 4), GridCoordinate::new(0, 1)]
        );
    }
}
