extern crate filelib;

use std::collections::{HashMap, VecDeque};

pub use filelib::load_no_blanks;
use gridlib::{Grid, GridCoordinate, GridPrintable, GridTraversable};
use itertools::Itertools;
use log::info;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum GridItem {
    Track,
    Start,
    End,
    Wall,
}

impl GridPrintable for GridItem {
    fn get_character(&self) -> char {
        match self {
            GridItem::Track => '.',
            GridItem::Start => 'S',
            GridItem::Wall => '#',
            GridItem::End => '#',
        }
    }
}

fn parse_map(string_list: &Vec<String>) -> (Grid<GridItem>, GridCoordinate, GridCoordinate) {
    let width = string_list.first().unwrap().len();
    let height = string_list.len();
    let mut values = vec![];
    let mut start = GridCoordinate::new(0, 0);
    let mut end = GridCoordinate::new(width - 1, height - 1);

    for (y, line) in string_list.into_iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    values.push(GridItem::Track);
                }
                '#' => {
                    values.push(GridItem::Wall);
                }
                'S' => {
                    start.x = x;
                    start.y = y;
                    values.push(GridItem::Start);
                }
                'E' => {
                    end.x = x;
                    end.y = y;
                    values.push(GridItem::End);
                }
                _ => panic!("Unknown char {}", c),
            }
        }
    }

    return (Grid::new(width, height, values), start, end);
}

/*/
fn print_path(grid: &Grid<GridItem>, path: Vec<SimpleGridOverlay>) {
    let lines = grid.grid_strings_with_overlay(path);
    for line in lines {
        info!("{}", line);
    }
    info!("----");
    info!("");
}
*/

// Normal path first
fn bfs(
    grid: &Grid<GridItem>,
    start: GridCoordinate,
    end: GridCoordinate,
    min_save: usize,
    max_cheat: usize,
) -> usize {
    let mut queue = VecDeque::new();
    let mut dists = HashMap::new();
    queue.push_back((start, 0usize));

    // Use BFS to find the end amount.
    while let Some((cur_coord, length)) = queue.pop_front() {
        if dists.contains_key(&cur_coord) {
            continue;
        }
        dists.insert(cur_coord, length);
        if cur_coord == end {
            info!("path found");
            continue;
        }

        let next_coords = grid.get_adjacent_coordinates(cur_coord);
        for next_coord in next_coords {
            let value = grid.get_value(next_coord).unwrap();
            if value != GridItem::Wall {
                queue.push_back((next_coord, length + 1));
            }
        }
    }
    let mut num_cheats = 0;
    for ((&coord_a, &distance_a), (&coord_b, &distance_b)) in dists.iter().tuple_combinations() {
        // Find the manhatten distance
        let distance = coord_a.y.abs_diff(coord_b.y) + coord_a.x.abs_diff(coord_b.x);
        // If the distance <= 2 we can cheat
        // if the distance between the two points are greater than distance we need to save then count it.
        if distance <= max_cheat && distance_b.abs_diff(distance_a) >= distance + min_save {
            num_cheats += 1;
        }
    }
    return num_cheats;
}

/// How many cheats will save >= 100 picoseconds?
/// ```
/// let vec1: Vec<String> = vec![
///     "###############",
///     "#...#...#.....#",
///     "#.#.#.#.#.###.#",
///     "#S#...#.#.#...#",
///     "#######.#.#.###",
///     "#######.#.#...#",
///     "#######.#.###.#",
///     "###..E#...#...#",
///     "###.#######.###",
///     "#...###...#...#",
///     "#.#####.#.###.#",
///     "#.#...#.#.#...#",
///     "#.#.#.#.#.#.###",
///     "#...#...#...###",
///     "###############"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day20::puzzle_a(&vec1, 2), 14+14+2+4+2+3+1+1+1+1+1);
/// ```
pub fn puzzle_a(string_list: &Vec<String>, min_save: usize) -> usize {
    let (grid, start, end) = parse_map(string_list);
    let normal_solution = bfs(&grid, start, end, min_save, 2);
    return normal_solution;
}

/// How many cheats will save >= 100 picoseconds, if yoou can cheat for 20?
/// ```
/// let vec1: Vec<String> = vec![
///     "###############",
///     "#...#...#.....#",
///     "#.#.#.#.#.###.#",
///     "#S#...#.#.#...#",
///     "#######.#.#.###",
///     "#######.#.#...#",
///     "#######.#.###.#",
///     "###..E#...#...#",
///     "###.#######.###",
///     "#...###...#...#",
///     "#.#####.#.###.#",
///     "#.#...#.#.#...#",
///     "#.#.#.#.#.#.###",
///     "#...#...#...###",
///     "###############"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day20::puzzle_b(&vec1, 50), 32+31+29+39+25+23+20+19+12+14+12+22+4+3);
/// ```
pub fn puzzle_b(string_list: &Vec<String>, min_save: usize) -> usize {
    let (grid, start, end) = parse_map(string_list);
    let normal_solution = bfs(&grid, start, end, min_save, 20);
    return normal_solution;
}
