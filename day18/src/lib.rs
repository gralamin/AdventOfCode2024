extern crate filelib;

use std::cmp;
use std::collections::HashSet;
use std::collections::VecDeque;

pub use filelib::load_no_blanks;
use gridlib::{Grid, GridCoordinate, GridPrintable, GridTraversable, SimpleGridOverlay};
use log::info;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum GridItem {
    Person,
    Corrupted,
    Safe,
}

impl GridPrintable for GridItem {
    fn get_character(&self) -> char {
        match self {
            GridItem::Person => '@',
            GridItem::Corrupted => '#',
            GridItem::Safe => '.',
        }
    }
}

fn parse_input(s: &Vec<String>) -> Vec<GridCoordinate> {
    let mut result = vec![];
    for line in s {
        let (x_s, y_s) = line.split_once(",").unwrap();
        let x = x_s.trim().parse().unwrap();
        let y = y_s.trim().parse().unwrap();
        let coord = GridCoordinate::new(x, y);
        result.push(coord);
    }
    return result;
}

fn bfs(
    grid: &Grid<GridItem>,
    start: GridCoordinate,
    end: GridCoordinate,
) -> Vec<SimpleGridOverlay> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let start_path = vec![start];
    queue.push_back((start, start_path));
    let mut best_path: Vec<SimpleGridOverlay> = vec![];

    while let Some((cur_coord, cur_path)) = queue.pop_front() {
        if visited.contains(&cur_coord) {
            continue;
        }
        visited.insert(cur_coord);
        if cur_coord == end {
            best_path = cur_path
                .into_iter()
                .map(|c| SimpleGridOverlay::new('O', c))
                .collect();
            break;
        }

        let next_coords = grid.get_adjacent_coordinates(cur_coord);
        for next_coord in next_coords {
            let value = grid.get_value(next_coord).unwrap();
            if value == GridItem::Safe {
                let mut new_path = cur_path.clone();
                new_path.push(next_coord);
                queue.push_back((next_coord, new_path));
            }
        }
    }

    return best_path;
}

fn print_path(grid: &Grid<GridItem>, path: Vec<SimpleGridOverlay>) {
    let lines = grid.grid_strings_with_overlay(path);
    for line in lines {
        info!("{}", line);
    }
    info!("----");
    info!("");
}

/// Find path from top left to bottom right, using the first x amount of input as walls.
/// ```
/// let vec1: Vec<String> = vec![
///     "5,4",
///     "4,2",
///     "4,5",
///     "3,0",
///     "2,1",
///     "6,3",
///     "2,4",
///     "1,5",
///     "0,6",
///     "3,3",
///     "2,6",
///     "5,1",
///     "1,2",
///     "5,5",
///     "2,5",
///     "6,5",
///     "1,4",
///     "0,4",
///     "6,4",
///     "1,1",
///     "6,1",
///     "1,0",
///     "0,5",
///     "1,6",
///     "2,0"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day18::puzzle_a(&vec1, 7, 7, 12), 22);
/// ```
pub fn puzzle_a(
    string_list: &Vec<String>,
    width: usize,
    height: usize,
    first_x_values: usize,
) -> usize {
    let mut values = vec![GridItem::Safe; width * height];
    let start = GridCoordinate::new(0, 0);
    values[start.x + start.y * width] = GridItem::Person;

    let bytes = parse_input(string_list);
    let size = cmp::min(first_x_values, values.len());
    for i in 0..size {
        let byte = bytes[i];
        values[byte.x + byte.y * width] = GridItem::Corrupted;
    }

    let grid = Grid::new(width, height, values);
    let path = bfs(&grid, start, GridCoordinate::new(width - 1, height - 1));
    print_path(&grid, path.clone());
    // We count start position in the len, so -1
    return path.len() - 1;
}

fn binary_search(
    bytes: Vec<GridCoordinate>,
    start_pos: usize,
    base_grid: &Grid<GridItem>,
) -> GridCoordinate {
    let length = bytes.len();
    let mut half = (length - start_pos) / 2;
    let mut hind = length - 1;
    let mut lind = start_pos;
    let mut smallest_bad = length - 1;
    let path_start = GridCoordinate::new(0, 0);
    let path_goal = GridCoordinate::new(base_grid.get_width() - 1, base_grid.get_height() - 1);

    while lind <= hind {
        let mut new_grid = base_grid.clone();
        for i in start_pos..=half {
            new_grid.set_value(bytes[i], GridItem::Corrupted);
        }
        let path = bfs(&new_grid, path_start, path_goal);
        if path.len() == 0 {
            // No path found, this is a bad
            smallest_bad = cmp::min(smallest_bad, half);
            // treat as greater than
            hind = half - 1;
        } else {
            info!("Found a path for {} bytes", half);
            print_path(&new_grid, path);
            lind = half + 1;
        }
        half = (hind + lind) / 2;
    }
    return bytes[smallest_bad];
}

/// Get Grid coordinate of first byte that makes no possible path
/// ```
/// let vec1: Vec<String> = vec![
///     "5,4",
///     "4,2",
///     "4,5",
///     "3,0",
///     "2,1",
///     "6,3",
///     "2,4",
///     "1,5",
///     "0,6",
///     "3,3",
///     "2,6",
///     "5,1",
///     "1,2",
///     "5,5",
///     "2,5",
///     "6,5",
///     "1,4",
///     "0,4",
///     "6,4",
///     "1,1",
///     "6,1",
///     "1,0",
///     "0,5",
///     "1,6",
///     "2,0"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day18::puzzle_b(&vec1, 7, 7, 12), (6,1));
/// ```
pub fn puzzle_b(
    string_list: &Vec<String>,
    width: usize,
    height: usize,
    start_pos: usize,
) -> (usize, usize) {
    let mut values = vec![GridItem::Safe; width * height];
    let start = GridCoordinate::new(0, 0);
    values[start.x + start.y * width] = GridItem::Person;

    let bytes = parse_input(string_list);
    let size = cmp::min(start_pos, values.len());
    for i in 0..size {
        let byte = bytes[i];
        values[byte.x + byte.y * width] = GridItem::Corrupted;
    }
    // We know 1 kilobyte has a solution, so start there.
    // Binary search between the rest of the length.

    let grid = Grid::new(width, height, values);
    let location = binary_search(bytes, start_pos, &grid);
    return (location.x, location.y);
}
