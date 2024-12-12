extern crate filelib;

use std::collections::{HashSet, VecDeque};

pub use filelib::load_no_blanks;
use gridlib::{Direction, Grid, GridCoordinate, GridTraversable};
use log::info;

fn parse(string_list: &Vec<String>) -> Grid<char> {
    let mut values = vec![];
    let height = string_list.len();
    let width = string_list.first().unwrap().len();
    for line in string_list {
        for c in line.chars() {
            values.push(c);
        }
    }
    return Grid::new(width, height, values);
}

type Cache = HashSet<GridCoordinate>;

// We can flood fill to calculate the area and permieter at the same time
// Flood fill is basically a BFS.
fn flood_fill(grid: &Grid<char>, visited: &mut Cache, coord: GridCoordinate) -> (usize, usize) {
    info!("Flood filling from {:?}", coord);
    let mut perimeter = 0;
    let mut area = 0;
    let mut queue = VecDeque::new();
    let matching_char = grid.get_value(coord).unwrap();
    queue.push_back(coord);

    while let Some(cur_coord) = queue.pop_front() {
        if visited.contains(&cur_coord) {
            continue;
        }
        visited.insert(cur_coord);
        area += 1;
        let max_coords = 4;
        let next_coords = grid.get_adjacent_coordinates(cur_coord);
        perimeter += max_coords - next_coords.len();
        for next_coord in next_coords {
            let value = grid.get_value(next_coord).unwrap();
            if value == matching_char {
                queue.push_back(next_coord);
            } else {
                perimeter += 1;
            }
        }
    }
    info!("Solution: area {}, perimeter {}", area, perimeter);
    return (area, perimeter);
}

/// Find price of fencing area, based on area * perimeter sum
/// ```
/// let vec1: Vec<String> = vec![
///     "AAAA",
///     "BBCD",
///     "BBCC",
///     "EEEC"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day12::puzzle_a(&vec1), 140);
/// ```
pub fn puzzle_a(string_list: &Vec<String>) -> usize {
    let grid = parse(string_list);
    let mut visited = Cache::new();
    let mut total_price = 0;
    for coord in grid.coord_iter() {
        if visited.contains(&coord) {
            continue;
        }
        let (area, perimeter) = flood_fill(&grid, &mut visited, coord);
        total_price += area * perimeter;
    }
    return total_price;
}

// We can flood fill to calculate the area and sides at the same time
// Flood fill is basically a BFS.
fn flood_fill_sides(
    grid: &Grid<char>,
    visited: &mut Cache,
    coord: GridCoordinate,
) -> (usize, usize) {
    info!("Flood filling from {:?}", coord);
    let mut sides: HashSet<(Direction, i64, i64)> = HashSet::new();
    let mut area = 0;
    let mut queue = VecDeque::new();
    let matching_char = grid.get_value(coord).unwrap();
    queue.push_back(coord);

    while let Some(cur_coord) = queue.pop_front() {
        if visited.contains(&cur_coord) {
            continue;
        }
        visited.insert(cur_coord);
        area += 1;
        let next_coords = grid.get_adjacent_coordinates_and_direction(cur_coord);
        // Handles case where A is alone on the map
        // Sides can be represented as follows: DIRECTION + Coord.x/y depending on direction, EG if there is a side to the EAST, we need to track the X coord. If there is one to the NORTH, we need to track the Y.
        let mut missed_directions = HashSet::new();
        missed_directions.insert(Direction::NORTH);
        missed_directions.insert(Direction::EAST);
        missed_directions.insert(Direction::SOUTH);
        missed_directions.insert(Direction::WEST);
        for (next_coord, direction) in next_coords {
            missed_directions.remove(&direction);
            let value = grid.get_value(next_coord).unwrap();
            if value == matching_char {
                queue.push_back(next_coord);
            } else {
                // add to the set based on direction
                // Note this over indexes, and creates a run we will eliminate in a bit.
                match direction {
                    Direction::NORTH | Direction::SOUTH => {
                        sides.insert((direction, next_coord.x as i64, next_coord.y as i64));
                    }
                    Direction::EAST | Direction::WEST => {
                        sides.insert((direction, next_coord.x as i64, next_coord.y as i64));
                    }
                    _ => panic!("Impossible scenario"),
                }
            }
        }
        // Handle missed directions - these are grid edge conditions, we can use a weird value to handle them (-1).
        // Consider an E
        // Each point in the E should be its on side
        // We can tell them apart by their Y coordinate.
        // But that creates runs on the west side.
        for direction in missed_directions {
            match direction {
                Direction::NORTH | Direction::SOUTH => {
                    sides.insert((direction, cur_coord.x as i64, -1));
                }
                Direction::EAST | Direction::WEST => {
                    sides.insert((direction, -1, cur_coord.y as i64));
                }
                _ => panic!("Impossible scenario"),
            }
        }
    }

    // We need to eliminate those runs
    for (side_dir, side_x, side_y) in sides.clone() {
        if !sides.contains(&(side_dir, side_x, side_y)) {
            // Already eliminated, continue
            continue;
        }
        match side_dir {
            Direction::NORTH | Direction::SOUTH => {
                // Check for any x adjacent to you, if there are, eliminate them now.
                // get all values directly adjacent to you.
                let mut adjacent_xs = HashSet::new();
                // go left first
                let mut left_values = side_x - 1;
                while left_values >= 0 {
                    if !sides.contains(&(side_dir, left_values, side_y)) {
                        break;
                    }
                    adjacent_xs.insert(left_values);
                    left_values -= 1;
                }

                // Now go right
                let mut right_values = side_x + 1;
                while right_values < grid.get_height() as i64 {
                    if !sides.contains(&(side_dir, right_values, side_y)) {
                        break;
                    }
                    adjacent_xs.insert(right_values);
                    right_values += 1;
                }

                for x in adjacent_xs {
                    sides.remove(&(side_dir, x, side_y));
                }
            }
            Direction::EAST | Direction::WEST => {
                // Check for any y adjacent to you, if there are, eliminate them now.
                // get all values directly adjacent to you.
                let mut adjacent_ys = HashSet::new();
                // go up first
                let mut above_values = side_y - 1;
                while above_values >= 0 {
                    if !sides.contains(&(side_dir, side_x, above_values)) {
                        break;
                    }
                    adjacent_ys.insert(above_values);
                    above_values -= 1;
                }

                // Now go down
                let mut below_values = side_y + 1;
                while below_values < grid.get_height() as i64 {
                    if !sides.contains(&(side_dir, side_x, below_values)) {
                        break;
                    }
                    adjacent_ys.insert(below_values);
                    below_values += 1;
                }

                for y in adjacent_ys {
                    sides.remove(&(side_dir, side_x, y));
                }
            }
            _ => panic!("Impossible case"),
        };
    }

    info!("Solution: area {}, sides {:?}", area, sides);
    return (area, sides.len());
}

/// Find price of fencing area, based on area * number of sides sum
/// ```
/// let vec1: Vec<String> = vec![
///     "AAAA",
///     "BBCD",
///     "BBCC",
///     "EEEC"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day12::puzzle_b(&vec1), 80);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> usize {
    let grid = parse(string_list);
    let mut visited = Cache::new();
    let mut total_price = 0;
    for coord in grid.coord_iter() {
        if visited.contains(&coord) {
            continue;
        }
        let (area, sides) = flood_fill_sides(&grid, &mut visited, coord);
        total_price += area * sides;
    }
    return total_price;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1_region_a() {
        let input = vec!["AAAA", "BBCD", "BBCC", "EEEC"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let grid = parse(&input);
        let mut visited = Cache::new();
        let (area, perimeter) = flood_fill(&grid, &mut visited, GridCoordinate::new(0, 0));
        assert_eq!(area, 4);
        assert_eq!(perimeter, 10);
    }

    #[test]
    fn test_example_1_region_b() {
        let input = vec!["AAAA", "BBCD", "BBCC", "EEEC"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let grid = parse(&input);
        let mut visited = Cache::new();
        let (area, perimeter) = flood_fill(&grid, &mut visited, GridCoordinate::new(0, 1));
        assert_eq!(area, 4);
        assert_eq!(perimeter, 8);
    }

    #[test]
    fn test_example_1_region_c() {
        let input = vec!["AAAA", "BBCD", "BBCC", "EEEC"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let grid = parse(&input);
        let mut visited = Cache::new();
        let (area, perimeter) = flood_fill(&grid, &mut visited, GridCoordinate::new(2, 1));
        assert_eq!(area, 4);
        assert_eq!(perimeter, 10);
    }

    #[test]
    fn test_example_1_region_d() {
        let input = vec!["AAAA", "BBCD", "BBCC", "EEEC"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let grid = parse(&input);
        let mut visited = Cache::new();
        let (area, perimeter) = flood_fill(&grid, &mut visited, GridCoordinate::new(3, 1));
        assert_eq!(area, 1);
        assert_eq!(perimeter, 4);
    }

    #[test]
    fn test_example_1_region_e() {
        let input = vec!["AAAA", "BBCD", "BBCC", "EEEC"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let grid = parse(&input);
        let mut visited = Cache::new();
        let (area, perimeter) = flood_fill(&grid, &mut visited, GridCoordinate::new(0, 3));
        assert_eq!(area, 3);
        assert_eq!(perimeter, 8);
    }

    #[test]
    fn test_example_2() {
        let input = vec!["OOOOO", "OXOXO", "OOOOO", "OXOXO", "OOOOO"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let result = puzzle_a(&input);
        assert_eq!(result, 772);
    }

    #[test]
    fn test_example_3() {
        let input = vec![
            "RRRRIICCFF",
            "RRRRIICCCF",
            "VVRRRCCFFF",
            "VVRCCCJFFF",
            "VVVVCJJCFE",
            "VVIVCCJJEE",
            "VVIIICJJEE",
            "MIIIIIJJEE",
            "MIIISIJEEE",
            "MMMISSJEEE",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let result = puzzle_a(&input);
        assert_eq!(result, 1930);
    }

    #[test]
    fn test_example_2_b() {
        let input = vec!["OOOOO", "OXOXO", "OOOOO", "OXOXO", "OOOOO"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let result = puzzle_b(&input);
        assert_eq!(result, 436);
    }

    #[test]
    fn test_example_3_b() {
        let input = vec!["EEEEE", "EXXXX", "EEEEE", "EXXXX", "EEEEE"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let result = puzzle_b(&input);
        assert_eq!(result, 236);
    }

    #[test]
    fn test_example_4_b() {
        let input = vec!["AAAAAA", "AAABBA", "AAABBA", "ABBAAA", "ABBAAA", "AAAAAA"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let result = puzzle_b(&input);
        assert_eq!(result, 368);
    }

    #[test]
    fn test_example_5b() {
        let input = vec![
            "RRRRIICCFF",
            "RRRRIICCCF",
            "VVRRRCCFFF",
            "VVRCCCJFFF",
            "VVVVCJJCFE",
            "VVIVCCJJEE",
            "VVIIICJJEE",
            "MIIIIIJJEE",
            "MIIISIJEEE",
            "MMMISSJEEE",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let result = puzzle_b(&input);
        assert_eq!(result, 1206);
    }

    #[test]
    fn test_example_1_region_a_partb() {
        let input = vec!["AAAA", "BBCD", "BBCC", "EEEC"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let grid = parse(&input);
        let mut visited = Cache::new();
        let (area, sides) = flood_fill_sides(&grid, &mut visited, GridCoordinate::new(0, 0));
        assert_eq!(area, 4);
        assert_eq!(sides, 4);
    }

    #[test]
    fn test_example_1_region_b_partb() {
        let input = vec!["AAAA", "BBCD", "BBCC", "EEEC"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let grid = parse(&input);
        let mut visited = Cache::new();
        let (area, sides) = flood_fill_sides(&grid, &mut visited, GridCoordinate::new(0, 1));
        assert_eq!(area, 4);
        assert_eq!(sides, 4);
    }

    #[test]
    fn test_example_1_region_c_partb() {
        let input = vec!["AAAA", "BBCD", "BBCC", "EEEC"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let grid = parse(&input);
        let mut visited = Cache::new();
        let (area, sides) = flood_fill_sides(&grid, &mut visited, GridCoordinate::new(2, 1));
        assert_eq!(area, 4);
        assert_eq!(sides, 8);
    }

    #[test]
    fn test_example_1_region_d_partb() {
        let input = vec!["AAAA", "BBCD", "BBCC", "EEEC"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let grid = parse(&input);
        let mut visited = Cache::new();
        let (area, sides) = flood_fill_sides(&grid, &mut visited, GridCoordinate::new(3, 1));
        assert_eq!(area, 1);
        assert_eq!(sides, 4);
    }

    #[test]
    fn test_example_1_region_e_partb() {
        let input = vec!["AAAA", "BBCD", "BBCC", "EEEC"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let grid = parse(&input);
        let mut visited = Cache::new();
        let (area, sides) = flood_fill_sides(&grid, &mut visited, GridCoordinate::new(0, 3));
        assert_eq!(area, 3);
        assert_eq!(sides, 4);
    }

    #[test]
    fn test_example_3_b_region_e() {
        let input = vec!["EEEEE", "EXXXX", "EEEEE", "EXXXX", "EEEEE"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let grid = parse(&input);
        let mut visited = Cache::new();
        let (area, sides) = flood_fill_sides(&grid, &mut visited, GridCoordinate::new(0, 0));
        assert_eq!(area, 17);
        assert_eq!(sides, 12);
        /*
        {(WEST, -1, 4) - E left line
        (NORTH, 3, -1) - E Top line
        (SOUTH, 3, -1) - E Bottom line

        (EAST, -1, 0) - E top prong
        (EAST, -1, 2) - E middle prong
        (EAST, -1, 4) - E Bottom prong

        (NORTH, 0, 1) - E empty space on line 1
        (SOUTH, 0, 1) - E empty space on line 1

        (NORTH, 0, 3) - E Empty space on bottom

        , , (EAST, 1, 0), , , (SOUTH, 0, 3)
         */
    }

    #[test]
    fn test_example_3_b_region_top_x() {
        let input = vec!["EEEEE", "EXXXX", "EEEEE", "EXXXX", "EEEEE"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let grid = parse(&input);
        let mut visited = Cache::new();
        let (area, sides) = flood_fill_sides(&grid, &mut visited, GridCoordinate::new(1, 1));
        assert_eq!(area, 4);
        assert_eq!(sides, 4);
    }

    #[test]
    fn test_example_3_b_region_bottom_x() {
        let input = vec!["EEEEE", "EXXXX", "EEEEE", "EXXXX", "EEEEE"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let grid = parse(&input);
        let mut visited = Cache::new();
        let (area, sides) = flood_fill_sides(&grid, &mut visited, GridCoordinate::new(1, 1));
        assert_eq!(area, 4);
        assert_eq!(sides, 4);
    }
}
