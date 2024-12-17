extern crate filelib;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::u32;

pub use filelib::load_no_blanks;
use gridlib::GridPrintable;
use gridlib::SimpleGridOverlay;
use gridlib::{Direction, Grid, GridCoordinate, GridTraversable};
use log::info;

type Map = Grid<Terrain>;
type Coord = GridCoordinate;
type PathStep = (Coord, Direction);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Terrain {
    Wall,
    Empty,
}

impl GridPrintable for Terrain {
    fn get_character(&self) -> char {
        return match self {
            Self::Wall => '#',
            Self::Empty => '.',
        };
    }
}

// Implement a custom Queue state to handle priority queue aspects
#[derive(Debug, Clone, Eq, PartialEq)]
struct QueueState {
    cur_direction: Direction,
    cur_cost: u32,
    cur_location: GridCoordinate,
    previous_steps: Vec<PathStep>,
}

impl Ord for QueueState {
    fn cmp(&self, other: &Self) -> Ordering {
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cur_cost
            .cmp(&self.cur_cost)
            .then_with(|| other.previous_steps.len().cmp(&self.previous_steps.len()))
            .then_with(|| other.cur_location.cmp(&self.cur_location))
    }
}

impl PartialOrd for QueueState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

// We need to get: Map, start location, end Location
fn parse_maze(strings: &Vec<String>) -> (Map, Coord, Coord) {
    let mut start_coord: Coord = Coord::new(0, 0);
    let mut end_coord: Coord = Coord::new(0, 0);
    let mut values = vec![];
    let height = strings.len();
    let width = strings.first().unwrap().len();

    for (y, line) in strings.into_iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    values.push(Terrain::Wall);
                }
                '.' => {
                    values.push(Terrain::Empty);
                }
                'S' => {
                    start_coord = Coord::new(x, y);
                    values.push(Terrain::Empty);
                }
                'E' => {
                    values.push(Terrain::Empty);
                    end_coord = Coord::new(x, y);
                }
                _ => panic!("Unknown character - Terrain {}", c),
            }
        }
    }

    let grid = Map::new(width, height, values);
    return (grid, start_coord, end_coord);
}

fn dijkstra_min_path(grid: Map, start: Coord, end: Coord) -> (usize, u32) {
    let start_path: Vec<PathStep> = vec![(start, Direction::EAST)];
    let mut final_path = vec![];
    let mut best_routes: HashSet<GridCoordinate> = HashSet::new();
    let mut best_cost = u32::MAX;
    // Set all distances to max
    // coordinate, direction
    let mut visited: HashSet<(GridCoordinate, Direction)> = HashSet::new();
    let mut dist: HashMap<(usize, Direction), u32> = HashMap::new();
    // There is a rule the start pos doesn't count.
    let width = grid.get_width();

    let mut queue = BinaryHeap::new();
    queue.push(QueueState {
        cur_direction: Direction::EAST,
        cur_cost: 0,
        cur_location: start,
        previous_steps: start_path,
    });
    while let Some(state) = queue.pop() {
        let cur_location = state.cur_location;
        let cur_cost = state.cur_cost;
        let cur_path = state.previous_steps;
        let cur_direction = state.cur_direction;
        let index = cur_location.y * width + cur_location.x;
        let key = (index, cur_direction);

        let location_cost = *dist.entry(key).or_insert(u32::MAX);
        if cur_cost < location_cost {
            *dist.entry(key).or_insert(u32::MAX) = cur_cost;
            if cur_location == end && cur_cost < best_cost {
                best_routes = HashSet::new();
            }
        }
        if cur_location == end && cur_cost <= best_cost {
            best_cost = cur_cost;
            let grid_coords: Vec<GridCoordinate> =
                cur_path.iter().map(|(coord, _)| *coord).collect();
            best_routes.extend(grid_coords);
            final_path = cur_path.clone();
            info!("Possible end path found!");
            let print_map = cur_path.into_iter().map(|(c, _)| c).collect();
            print_best_steps(&grid, &print_map);
            continue;
        } else if cur_location == end {
            info!("Throwing out route: {:?}, cost: {:?}", cur_path, cur_cost);
            let print_map = cur_path.into_iter().map(|(c, _)| c).collect();
            print_best_steps(&grid, &print_map);
            continue;
        }
        if visited.contains(&(cur_location, cur_direction)) && location_cost < cur_cost {
            continue;
        }
        visited.insert((cur_location, cur_direction));
        // Spin in place options first
        for &potential_direction in Direction::cardinal_iterator() {
            if potential_direction == cur_direction {
                continue;
            }
            let mut new_path = cur_path.clone();
            new_path.push((cur_location, potential_direction));
            queue.push(QueueState {
                cur_direction: potential_direction,
                cur_cost: cur_cost + 1000,
                cur_location: cur_location,
                previous_steps: new_path,
            });
        }
        // Try going forward
        let next_coord = grid
            .get_coordinate_by_direction(cur_location, cur_direction)
            .unwrap();
        let next_value = grid.get_value(next_coord).unwrap();
        if next_value == Terrain::Wall {
            continue;
        }
        let mut new_path = cur_path.clone();
        new_path.push((next_coord, cur_direction));
        queue.push(QueueState {
            cur_direction: cur_direction,
            cur_cost: cur_cost + 1,
            cur_location: next_coord,
            previous_steps: new_path,
        });
    }
    info!("Final path: {:?}", final_path);
    print_best_steps(&grid, &best_routes);
    return (best_routes.len(), best_cost);
}

/// S -> E in a maze
/// ```
/// let vec1: Vec<String> = vec![
///     "###############",
///     "#.......#....E#",
///     "#.#.###.#.###.#",
///     "#.....#.#...#.#",
///     "#.###.#####.#.#",
///     "#.#.#.......#.#",
///     "#.#.#####.###.#",
///     "#...........#.#",
///     "###.#.#####.#.#",
///     "#...#.....#.#.#",
///     "#.#.#.###.#.#.#",
///     "#.....#...#.#.#",
///     "#.###.#.#.#.#.#",
///     "#S..#.....#...#",
///     "###############"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day16::puzzle_a(&vec1), 7036);
/// ```
pub fn puzzle_a(string_list: &Vec<String>) -> u32 {
    let (grid, start_coord, end_coord) = parse_maze(string_list);
    let (_, cost) = dijkstra_min_path(grid, start_coord, end_coord);
    return cost;
}

fn print_best_steps(map: &Map, best_route: &HashSet<Coord>) {
    let overlay = best_route.iter().map(|x| SimpleGridOverlay::new('O', *x));
    let lines = map.grid_strings_with_overlay(overlay);
    for line in lines {
        info!("{}", line);
    }
    info!("----");
    info!("");
}

/// As above, but find the number of tiles that are part of the best path through the maze
/// ```
/// let vec1: Vec<String> = vec![
///     "###############",
///     "#.......#....E#",
///     "#.#.###.#.###.#",
///     "#.....#.#...#.#",
///     "#.###.#####.#.#",
///     "#.#.#.......#.#",
///     "#.#.#####.###.#",
///     "#...........#.#",
///     "###.#.#####.#.#",
///     "#...#.....#.#.#",
///     "#.#.#.###.#.#.#",
///     "#.....#...#.#.#",
///     "#.###.#.#.#.#.#",
///     "#S..#.....#...#",
///     "###############"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day16::puzzle_b(&vec1), 45);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> usize {
    let (grid, start_coord, end_coord) = parse_maze(string_list);
    let (num_best_paths, _) = dijkstra_min_path(grid, start_coord, end_coord);
    return num_best_paths;
}
