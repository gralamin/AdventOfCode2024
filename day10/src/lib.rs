extern crate filelib;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

pub use filelib::load_no_blanks;
use gridlib::Grid;
use gridlib::GridCoordinate;
use gridlib::GridTraversable;
use log::info;

fn parse_topgraphic_map(string_list: &Vec<String>) -> Grid<u32> {
    let mut values = vec![];
    let width = string_list.first().unwrap().len();
    let height = string_list.len();
    for s in string_list.iter() {
        for c in s.chars() {
            let v = c.to_digit(10).unwrap();
            values.push(v);
        }
    }

    return Grid::new(width, height, values);
}

// Trails start at height 0, end at 9 and increase by 1 step at a time.
// Never diagnonal
// Return the trails - start at 0, end at 9.
fn find_trails(grid: &Grid<u32>) -> Vec<Vec<GridCoordinate>> {
    let mut results = vec![];

    let mut zeroes = vec![];
    let mut nines = vec![];
    for coord in grid.coord_iter() {
        let value = grid.get_value(coord).unwrap();
        if value == 0 {
            zeroes.push(coord);
        } else if value == 9 {
            nines.push(coord);
        }
    }

    for zero in zeroes {
        for nine in nines.clone() {
            // Source, path, target
            let mut queue = VecDeque::new();
            queue.push_back((zero, vec![], nine));

            while !queue.is_empty() {
                let (current, path, target) = queue.pop_front().unwrap();
                let mut next_path = path.clone();
                next_path.push(current);
                if current == target {
                    results.push(next_path);
                    info!("Path between {} and {}", zero, nine);
                    // There may be more than one path...
                    continue;
                }
                let value = grid.get_value(current).unwrap();
                let next_value = value + 1;
                for coord in grid.get_adjacent_coordinates(current) {
                    if grid.get_value(coord).unwrap() == next_value {
                        queue.push_back((coord, next_path.clone(), target));
                    }
                }
            }
            info!("No path between {} and {}", zero, nine);
        }
    }

    return results;
}

fn score_trails(trails: Vec<Vec<GridCoordinate>>) -> u32 {
    // The score is the number of unique endpoints for each start point.
    let mut map: HashMap<GridCoordinate, HashSet<GridCoordinate>> = HashMap::new();
    for trail in trails {
        let first = trail.first().unwrap();
        let last = trail.last().unwrap();
        let hash_set = map.get_mut(first);
        match hash_set {
            Some(hash_set_literal) => {
                hash_set_literal.insert(*last);
            }
            None => {
                let mut hash_set: HashSet<GridCoordinate> = HashSet::new();
                hash_set.insert(*last);
                map.insert(*first, hash_set);
            }
        }
    }
    return map.values().map(|s| s.len()).sum::<usize>() as u32;
}

/// score the trailheads
/// ```
/// let vec1: Vec<String> = vec![
///     "89010123",
///     "78121874",
///     "87430965",
///     "96549874",
///     "45678903",
///     "32019012",
///     "01329801",
///     "10456732"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day10::puzzle_a(&vec1), 36);
/// ```
pub fn puzzle_a(string_list: &Vec<String>) -> u32 {
    let map = parse_topgraphic_map(string_list);
    let trails = find_trails(&map);
    return score_trails(trails);
}

fn rate_trails(trails: Vec<Vec<GridCoordinate>>) -> u32 {
    //The rating is the number of distinct trails that start from there.
    let mut map: HashMap<GridCoordinate, HashSet<Vec<GridCoordinate>>> = HashMap::new();
    for trail in trails {
        let first = trail.first().unwrap();
        let hash_set = map.get_mut(first);
        match hash_set {
            Some(hash_set_literal) => {
                hash_set_literal.insert(trail);
            }
            None => {
                let mut hash_set: HashSet<Vec<GridCoordinate>> = HashSet::new();
                hash_set.insert(trail.clone());
                map.insert(*first, hash_set);
            }
        }
    }
    return map.values().map(|s| s.len()).sum::<usize>() as u32;
}

/// Determine rating instead.
/// ```
/// let vec1: Vec<String> = vec![
///     "89010123",
///     "78121874",
///     "87430965",
///     "96549874",
///     "45678903",
///     "32019012",
///     "01329801",
///     "10456732"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day10::puzzle_b(&vec1), 81);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> u32 {
    let map = parse_topgraphic_map(string_list);
    let trails = find_trails(&map);
    return rate_trails(trails);
}
