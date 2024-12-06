extern crate filelib;

pub use filelib::load_no_blanks;
use gridlib::GridTraversable;
use gridlib::{Direction, Grid, GridCoordinate};
use log::info;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Guard {
    facing: Direction,
    position: GridCoordinate,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum SpaceType {
    Empty,
    Wall,
}

type Map = Grid<SpaceType>;

impl Guard {
    fn new(direction: Direction, position: GridCoordinate) -> Guard {
        return Guard {
            facing: direction,
            position: position,
        };
    }

    fn rotate(&mut self) {
        // Rotate right -> North to EAST
        self.facing = match self.facing {
            Direction::NORTH => Direction::EAST,
            Direction::EAST => Direction::SOUTH,
            Direction::SOUTH => Direction::WEST,
            Direction::WEST => Direction::NORTH,
            _ => panic!("Not covered"),
        }
    }

    fn step(&mut self, g: &Map) {
        let next_coord = g.get_coordinate_by_direction(self.position, self.facing);
        match next_coord {
            Some(x) => {
                let next_value = g.get_value(x).unwrap();
                if next_value == SpaceType::Wall {
                    self.rotate()
                } else {
                    self.position = x
                }
            }
            _ => {
                // Stay stuck to signal done moving
            }
        }
    }
}

fn parse_map(string_list: &Vec<String>) -> (Map, Guard) {
    let mut values = vec![];
    let mut guard_pos = GridCoordinate::new(0, 0);
    let mut guard_dir = Direction::NORTH;
    let width = string_list.first().unwrap().len();
    let height = string_list.len();
    for (y, line) in string_list.into_iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let space: SpaceType;
            match c {
                '.' => space = SpaceType::Empty,
                '#' => space = SpaceType::Wall,
                '^' => {
                    info!("Found guard at {},{}", x, y);
                    guard_dir = Direction::NORTH;
                    guard_pos = GridCoordinate::new(x, y);
                    space = SpaceType::Empty
                }
                '>' => {
                    info!("Found guard at {},{}", x, y);
                    guard_dir = Direction::EAST;
                    guard_pos = GridCoordinate::new(x, y);
                    space = SpaceType::Empty
                }
                'V' => {
                    info!("Found guard at {},{}", x, y);
                    guard_dir = Direction::SOUTH;
                    guard_pos = GridCoordinate::new(x, y);
                    space = SpaceType::Empty
                }
                '<' => {
                    info!("Found guard at {},{}", x, y);
                    guard_dir = Direction::WEST;
                    guard_pos = GridCoordinate::new(x, y);
                    space = SpaceType::Empty
                }
                _ => panic!("Unknown character"),
            };
            values.push(space);
        }
    }
    return (
        Map::new(width, height, values),
        Guard::new(guard_dir, guard_pos),
    );
}

fn cycle_guard(m: &Map, g: &mut Guard) -> HashSet<GridCoordinate> {
    let mut positions = HashSet::new();
    positions.insert(g.position);
    let mut last_pos = g.position.clone();
    let mut last_dir = g.facing.clone();
    g.step(m);
    while g.position != last_pos || g.facing != last_dir {
        positions.insert(g.position);
        last_pos = g.position.clone();
        last_dir = g.facing.clone();
        g.step(m);
    }
    return positions;
}

/// Figure out all the squares the guard will be in by raytracing and reflecting
/// ```
/// let vec1: Vec<String> = vec![
///     "....#.....",
///     ".........#",
///     "..........",
///     "..#.......",
///     ".......#..",
///     "..........",
///     ".#..^.....",
///     "........#.",
///     "#.........",
///     "......#..."
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day06::puzzle_a(&vec1), 41);
/// ```
pub fn puzzle_a(string_list: &Vec<String>) -> usize {
    let (map, mut guard) = parse_map(string_list);
    let positions = cycle_guard(&map, &mut guard);
    return positions.len();
}

fn cycle_guard_detect_loop(m: &Map, g: &mut Guard) -> bool {
    let mut last_pos = g.position.clone();
    let mut last_dir = g.facing.clone();
    let mut previous_location = HashSet::new();
    previous_location.insert((last_dir, last_pos));
    g.step(m);

    while g.position != last_pos || g.facing != last_dir {
        if previous_location.contains(&(g.facing, g.position)) {
            return true;
        }
        last_pos = g.position.clone();
        last_dir = g.facing.clone();
        previous_location.insert((last_dir, last_pos));
        g.step(m);
    }
    return false;
}

/// Flip a bit to trap the guard in a loop.
/// ```
/// let vec1: Vec<String> = vec![
///     "....#.....",
///     ".........#",
///     "..........",
///     "..#.......",
///     ".......#..",
///     "..........",
///     ".#..^.....",
///     "........#.",
///     "#.........",
///     "......#..."
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day06::puzzle_b(&vec1), 6);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> usize {
    let (map, guard) = parse_map(string_list);
    let mut clone_iniital_path_guard = guard.clone();
    let positions = cycle_guard(&map, &mut clone_iniital_path_guard);

    return map
        .coord_iter()
        .filter(|&coord| {
            if coord == guard.position {
                // Can't put in starting position
                return false;
            }
            if !positions.contains(&coord) {
                // If we never reach it, no point in checking
                return false;
            }
            // Valid coord to switch
            let mut cur_map = map.clone();
            cur_map.set_value(coord, SpaceType::Wall);
            let mut cur_guard = guard.clone();
            return cycle_guard_detect_loop(&cur_map, &mut cur_guard);
        })
        .collect::<Vec<_>>()
        .len();
}
