extern crate filelib;

use std::collections::{HashMap, VecDeque};

pub use filelib::load_no_blanks;
use gridlib::{Direction, Grid, GridCoordinate, GridTraversable};
use log::info;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Button {
    Number(usize),
    Activate,
    Invalid,
    DirButton(Direction),
}

type Cache = HashMap<(Button, Button, usize), usize>;

// In example num levels is 3
// level 3 = key pad
// level 2 = robot 1
// level 1 = robot 2
// level 0 = you
fn key_pad_get_path_from_button_to_button(
    s: Button,
    e: Button,
    num_levels: usize,
    cache: &mut Cache,
) -> usize {
    // 7, 8, 9
    // 4, 5, 6
    // 1, 2, 3
    // _, 0, A
    let grid = Grid::new(
        3,
        4,
        vec![
            Button::Number(7),
            Button::Number(8),
            Button::Number(9),
            Button::Number(4),
            Button::Number(5),
            Button::Number(6),
            Button::Number(1),
            Button::Number(2),
            Button::Number(3),
            Button::Invalid,
            Button::Number(0),
            Button::Activate,
        ],
    );
    let mut queue: VecDeque<(GridCoordinate, Vec<Direction>)> = VecDeque::new();
    let mut cheapest_length = usize::MAX;

    // For this BFS instead of visited, we just always want to move towards the goal
    // which will filter us instead.
    let mut start_location = GridCoordinate::new(0, 0);
    let mut end_location = GridCoordinate::new(0, 0);
    for c in grid.coord_iter() {
        let v = grid.get_value(c).unwrap();
        if v == s {
            start_location = c;
        }
        if v == e {
            end_location = c;
        }
    }

    queue.push_back((start_location, vec![]));
    while let Some((cur_location, cur_path)) = queue.pop_front() {
        let cur_value = grid.get_value(cur_location).unwrap();

        if cur_value == e {
            let mut required_path: Vec<Button> =
                cur_path.iter().map(|&d| Button::DirButton(d)).collect();
            required_path.push(Button::Activate);
            let indirect_path =
                cheapest_indirect_path(required_path.clone(), num_levels - 1, cache);
            if indirect_path < cheapest_length {
                cheapest_length = indirect_path;
            }
            continue;
        }

        for (potential_coord, direction) in
            grid.get_adjacent_coordinates_and_direction(cur_location)
        {
            if direction == Direction::EAST && end_location.x <= cur_location.x {
                // Don't go further east, we know it won't work.
                // East increments X, so this moves us awway
                continue;
            } else if direction == Direction::WEST && end_location.x >= cur_location.x {
                // Don't go further west, we know it won't work.
                // West decrements X, so this moves us awway
                continue;
            } else if direction == Direction::NORTH && end_location.y >= cur_location.y {
                // Don't go further north, we know it won't work.
                // North decrements X, so this moves us awway
                continue;
            } else if direction == Direction::SOUTH && end_location.y <= cur_location.y {
                // Don't go further south, we know it won't work.
                // south increments X, so this moves us awway
                continue;
            }

            let value = grid.get_value(potential_coord).unwrap();
            if value != Button::Invalid {
                let mut path = cur_path.clone();
                path.push(direction);
                queue.push_back((potential_coord, path));
            }
        }
    }
    return cheapest_length;
}

fn cheapest_indirect_path(required_path: Vec<Button>, level: usize, cache: &mut Cache) -> usize {
    // level 0 is you
    if level == 0 {
        return required_path.len();
    }

    let start = Button::Activate;
    let mut last_button = start;
    let mut total_length = 0;
    for button in required_path {
        let key = (last_button, button, level);
        if cache.contains_key(&key) {
            let cached = cache.get(&key).unwrap();
            total_length += cached;
        } else {
            let path =
                direction_key_pad_get_path_from_button_to_button(last_button, button, level, cache);
            total_length += path;
            cache.insert(key, path);
        }
        last_button = button;
    }

    return total_length;
}

fn direction_key_pad_get_path_from_button_to_button(
    s: Button,
    e: Button,
    level: usize,
    cache: &mut Cache,
) -> usize {
    // _, North, A
    // West, South, East
    let grid = Grid::new(
        3,
        2,
        vec![
            Button::Invalid,
            Button::DirButton(Direction::NORTH),
            Button::Activate,
            Button::DirButton(Direction::WEST),
            Button::DirButton(Direction::SOUTH),
            Button::DirButton(Direction::EAST),
        ],
    );
    let mut queue: VecDeque<(GridCoordinate, Vec<Direction>)> = VecDeque::new();
    let mut cheapest_cost = usize::MAX;

    // For this BFS instead of visited, we just always want to move towards the goal
    // which will filter us instead.
    let mut start_location = GridCoordinate::new(0, 0);
    let mut end_location = GridCoordinate::new(0, 0);
    for c in grid.coord_iter() {
        let v = grid.get_value(c).unwrap();
        if v == s {
            start_location = c;
        }
        if v == e {
            end_location = c;
        }
    }

    queue.push_back((start_location, vec![]));
    while let Some((cur_location, cur_path)) = queue.pop_front() {
        let cur_value: Button = grid.get_value(cur_location).unwrap();

        if cur_value == e {
            let mut required_path: Vec<Button> =
                cur_path.iter().map(|&d| Button::DirButton(d)).collect();
            required_path.push(Button::Activate);
            let path = cheapest_indirect_path(required_path.clone(), level - 1, cache);
            if path < cheapest_cost {
                cheapest_cost = path;
            }
            continue;
        }

        for (potential_coord, direction) in
            grid.get_adjacent_coordinates_and_direction(cur_location)
        {
            if direction == Direction::EAST && end_location.x <= cur_location.x {
                // Don't go further east, we know it won't work.
                continue;
            } else if direction == Direction::WEST && end_location.x >= cur_location.x {
                // Don't go further west, we know it won't work.
                continue;
            } else if direction == Direction::NORTH && end_location.y >= cur_location.y {
                // Don't go further north, we know it won't work.
                continue;
            } else if direction == Direction::SOUTH && end_location.y <= cur_location.y {
                // Don't go further south, we know it won't work.
                continue;
            }

            let value = grid.get_value(potential_coord).unwrap();
            if value != Button::Invalid {
                let mut path = cur_path.clone();
                path.push(direction);
                queue.push_back((potential_coord, path));
            }
        }
    }
    return cheapest_cost;
}

fn parse_codes(string_list: &Vec<String>) -> Vec<Vec<Button>> {
    let mut codes = vec![];

    for line in string_list {
        let mut current = vec![];
        for c in line.chars() {
            current.push(match c {
                '0' => Button::Number(0),
                '1' => Button::Number(1),
                '2' => Button::Number(2),
                '3' => Button::Number(3),
                '4' => Button::Number(4),
                '5' => Button::Number(5),
                '6' => Button::Number(6),
                '7' => Button::Number(7),
                '8' => Button::Number(8),
                '9' => Button::Number(9),
                'A' => Button::Activate,
                _ => panic!("Invalid char {}", c),
            });
        }
        codes.push(current);
    }

    return codes;
}

/// Find the complexity of the buttons
/// ```
/// let vec1: Vec<String> = vec![
///     "029A",
///     "980A",
///     "179A",
///     "456A",
///     "379A"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day21::puzzle_a(&vec1), 126384);
/// ```
pub fn puzzle_a(string_list: &Vec<String>) -> usize {
    let mut cache: Cache = HashMap::new();
    let start = Button::Activate;
    let codes = parse_codes(string_list);
    let num_indirection: usize = 2;
    // Increment by 1, that way we count for yourself and bottom robot
    // Since this will count down to 0 to represent you
    let levels = num_indirection + 1;
    let mut final_paths_len: Vec<usize> = vec![];
    let mut final_numbers = vec![];

    for code in codes {
        let mut last_button = start;
        let mut path = 0;
        let mut numeric_part: usize = 0;
        for button in code {
            let so_far =
                key_pad_get_path_from_button_to_button(last_button, button, levels, &mut cache);
            match button {
                Button::Number(x) => numeric_part = numeric_part * 10 + x,
                _ => {
                    // Do nothing
                }
            };
            path += so_far;
            last_button = button;
        }
        final_paths_len.push(path);
        final_numbers.push(numeric_part);
    }

    let mut sum = 0;
    info!(
        "num len {}, path len {}",
        final_numbers.len(),
        final_paths_len.len()
    );

    for (path, num) in final_paths_len.into_iter().zip(final_numbers) {
        info!("num: {}, Path: {:?}", num, path);
        sum += num * path;
    }
    return sum;
}

/// Find the complexity of the buttons with 26 levels
/// ```
/// let vec1: Vec<String> = vec![
///     "029A",
///     "980A",
///     "179A",
///     "456A",
///     "379A"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day21::puzzle_b(&vec1), 154115708116294);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> usize {
    let mut cache: Cache = HashMap::new();
    let start = Button::Activate;
    let codes = parse_codes(string_list);
    let num_indirection: usize = 25;
    // Increment by 1, that way we count for yourself and bottom robot
    // Since this will count down to 0 to represent you
    let levels = num_indirection + 1;
    let mut final_path_len: Vec<usize> = vec![];
    let mut final_numbers = vec![];

    for code in codes {
        let mut last_button = start;
        let mut path_length = 0;
        let mut numeric_part: usize = 0;
        for button in code {
            let so_far =
                key_pad_get_path_from_button_to_button(last_button, button, levels, &mut cache);
            match button {
                Button::Number(x) => numeric_part = numeric_part * 10 + x,
                _ => {
                    // Do nothing
                }
            };
            path_length += so_far;
            last_button = button;
        }
        final_path_len.push(path_length);
        final_numbers.push(numeric_part);
    }

    let mut sum = 0;
    info!(
        "num len {}, path len {}",
        final_numbers.len(),
        final_path_len.len()
    );

    for (path, num) in final_path_len.into_iter().zip(final_numbers) {
        info!("num: {}, Path size: {:?}", num, path);
        sum += num * path;
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_to_0_no_indirect() {
        let mut cache = Cache::new();
        let start = Button::Activate;
        let end = Button::Number(0);
        let v = key_pad_get_path_from_button_to_button(start, end, 1, &mut cache);
        assert_eq!(
            v,
            vec![Button::DirButton(Direction::WEST), Button::Activate].len()
        );
    }

    #[test]
    fn test_a_to_0_one_indirect() {
        let mut cache = Cache::new();
        let start = Button::Activate;
        let end = Button::Number(0);
        let v = key_pad_get_path_from_button_to_button(start, end, 2, &mut cache);
        assert_eq!(
            v,
            vec![
                Button::DirButton(Direction::SOUTH),
                Button::DirButton(Direction::WEST),
                Button::DirButton(Direction::WEST),
                Button::Activate,
                Button::DirButton(Direction::EAST),
                Button::DirButton(Direction::NORTH), // At this level, EEN and ENE are equivalent
                Button::DirButton(Direction::EAST),
                Button::Activate
            ]
            .len()
        );
    }

    #[test]
    fn test_a_to_0_two_indirect() {
        let mut cache = Cache::new();
        let start = Button::Activate;
        let end = Button::Number(0);
        let v = key_pad_get_path_from_button_to_button(start, end, 3, &mut cache);
        assert_eq!(
            v,
            vec![
                Button::DirButton(Direction::SOUTH),
                Button::DirButton(Direction::WEST),
                Button::Activate,
                Button::DirButton(Direction::WEST),
                Button::Activate,
                Button::Activate,
                Button::DirButton(Direction::EAST),
                Button::DirButton(Direction::NORTH),
                Button::DirButton(Direction::EAST),
                Button::Activate,
                Button::DirButton(Direction::SOUTH),
                Button::Activate,
                Button::Activate,
                Button::DirButton(Direction::NORTH),
                Button::DirButton(Direction::WEST),
                Button::Activate,
                Button::DirButton(Direction::EAST),
                Button::Activate,
            ]
            .len()
        );
    }

    /*
    <vA
    <AA
    >>^A
    vAA
    <^A
    >A

    leftover:
    <v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
     */

    #[test]
    fn test_0_to_2_no_indirect() {
        let mut cache = Cache::new();
        let start = Button::Number(0);
        let end = Button::Number(2);
        let v = key_pad_get_path_from_button_to_button(start, end, 1, &mut cache);
        assert_eq!(
            v,
            vec![Button::DirButton(Direction::NORTH), Button::Activate].len()
        );
    }

    #[test]
    fn test_0_to_2_one_indirect() {
        let mut cache = Cache::new();
        let start = Button::Number(0);
        let end = Button::Number(2);
        let v = key_pad_get_path_from_button_to_button(start, end, 2, &mut cache);
        assert_eq!(
            v,
            vec![
                Button::DirButton(Direction::WEST),
                Button::Activate,
                Button::DirButton(Direction::EAST),
                Button::Activate
            ]
            .len()
        );
    }

    #[test]
    fn test_2_to_9_no_indirect() {
        let mut cache = Cache::new();
        let start = Button::Number(2);
        let end = Button::Number(9);
        let v = key_pad_get_path_from_button_to_button(start, end, 1, &mut cache);
        // Is this worse than ENN?
        assert_eq!(
            v,
            vec![
                Button::DirButton(Direction::NORTH),
                Button::DirButton(Direction::NORTH),
                Button::DirButton(Direction::EAST),
                Button::Activate
            ]
            .len()
        );
    }

    #[test]
    fn test_2_to_9_one_indirect() {
        let mut cache = Cache::new();
        let start = Button::Number(2);
        let end = Button::Number(9);
        let v = key_pad_get_path_from_button_to_button(start, end, 2, &mut cache);
        // Is this worse than ENN?
        assert_eq!(
            v,
            vec![
                Button::DirButton(Direction::WEST),
                Button::Activate,
                Button::Activate,
                Button::DirButton(Direction::EAST),
                Button::DirButton(Direction::SOUTH),
                Button::Activate,
                Button::DirButton(Direction::NORTH),
                Button::Activate,
            ]
            .len()
        );
    }

    #[test]
    fn test_9_to_a_no_indirect() {
        let mut cache = Cache::new();
        let start = Button::Number(9);
        let end = Button::Activate;
        let v = key_pad_get_path_from_button_to_button(start, end, 1, &mut cache);
        assert_eq!(
            v,
            vec![
                Button::DirButton(Direction::SOUTH),
                Button::DirButton(Direction::SOUTH),
                Button::DirButton(Direction::SOUTH),
                Button::Activate
            ]
            .len()
        );
    }

    #[test]
    fn test_9_to_a_one_indirect() {
        let mut cache = Cache::new();
        let start = Button::Number(9);
        let end = Button::Activate;
        let v = key_pad_get_path_from_button_to_button(start, end, 2, &mut cache);
        assert_eq!(
            v,
            vec![
                Button::DirButton(Direction::SOUTH),
                Button::DirButton(Direction::WEST),
                Button::Activate,
                Button::Activate,
                Button::Activate,
                Button::DirButton(Direction::NORTH),
                Button::DirButton(Direction::EAST),
                Button::Activate
            ]
            .len()
        );
    }
}
