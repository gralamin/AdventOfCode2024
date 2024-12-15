extern crate filelib;

use std::collections::{HashSet, VecDeque};

pub use filelib::{load, split_lines_by_blanks};
use gridlib::{Direction, Grid, GridCoordinate, GridTraversable};
use log::info;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Terrain {
    Wall,
    Empty,
}

type Map = Grid<Terrain>;
type Coord = GridCoordinate;

// We need to get: Map, Robot location (@), Box (O)
fn parse_warehouse(strings: &Vec<String>) -> (Map, Coord, Vec<Coord>) {
    let mut robot_coord: Coord = Coord::new(0, 0);
    let mut values = vec![];
    let height = strings.len();
    let width = strings.first().unwrap().len();
    let mut boxes = vec![];

    for (y, line) in strings.into_iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    values.push(Terrain::Wall);
                }
                '.' => {
                    values.push(Terrain::Empty);
                }
                '@' => {
                    robot_coord = Coord::new(x, y);
                    values.push(Terrain::Empty);
                }
                'O' => {
                    values.push(Terrain::Empty);
                    boxes.push(Coord::new(x, y));
                }
                _ => panic!("Unknown character - Terrain {}", c),
            }
        }
    }

    let grid = Map::new(width, height, values);
    return (grid, robot_coord, boxes);
}

fn parse_moves(strings: &Vec<String>) -> Vec<Direction> {
    let mut results = vec![];
    for line in strings {
        for c in line.chars() {
            results.push(match c {
                '<' => Direction::WEST,
                '^' => Direction::NORTH,
                'V' => Direction::SOUTH,
                'v' => Direction::SOUTH,
                '>' => Direction::EAST,
                _ => panic!("Unknown character - Direction {}", c),
            });
        }
    }
    return results;
}

fn step(map: &Map, robot: Coord, boxes: &Vec<Coord>, direction: Direction) -> (Coord, Vec<Coord>) {
    let mut new_robot = robot.clone();
    let mut new_boxes = vec![];

    let potential_new_coord = map.get_coordinate_by_direction(robot, direction).unwrap();
    info!("Checking if can go to {}", potential_new_coord);
    let potential_value = map.get_value(potential_new_coord).unwrap();
    match potential_value {
        Terrain::Wall => {
            // No moves to do, moving into a wall
            new_boxes = boxes.clone();
        }
        Terrain::Empty => {
            // potentially valid move!
            // check if there is a box here
            let mut boxes_to_move: Vec<Coord> = vec![];
            let matching_box = boxes.iter().find(|&&x| x == potential_new_coord);
            if let Some(box_value) = matching_box {
                // find all boxes in a row / column from here, until we hit an emtpy space, or a wall.
                boxes_to_move.push(*box_value);
                let mut train_coord = potential_new_coord;
                let mut train_value;
                let mut good_to_move = true;
                while good_to_move {
                    train_coord = map
                        .get_coordinate_by_direction(train_coord, direction)
                        .unwrap();
                    train_value = map.get_value(train_coord).unwrap();
                    match train_value {
                        Terrain::Wall => {
                            // Hit a wall
                            good_to_move = false;
                            break;
                        }
                        Terrain::Empty => {
                            // check if we have a box
                            let train_box = boxes.iter().find(|&&x| x == train_coord);
                            if let Some(found_box) = train_box {
                                boxes_to_move.push(*found_box);
                            } else {
                                // No box, we are good to move
                                break;
                            }
                        }
                    };
                }
                if good_to_move {
                    new_boxes.extend(boxes.iter().filter(|&x| !boxes_to_move.contains(x)));
                    for cur_box in boxes_to_move {
                        new_boxes
                            .push(map.get_coordinate_by_direction(cur_box, direction).unwrap());
                    }
                    new_robot = potential_new_coord;
                } else {
                    info!("Invalid move case");
                    // Invalid move
                    new_boxes = boxes.clone();
                    new_robot = robot.clone();
                }
            } else {
                info!("Just move");
                // No box, just move
                new_boxes = boxes.clone();
                new_robot = potential_new_coord;
            }
        }
    }

    info!("Robot is now at: {}", new_robot);
    return (new_robot, new_boxes);
}

/// Find all boxes GPS after move
/// ```
/// let vec1: Vec<Vec<String>> = vec![vec![
///     "##########",
///     "#..O..O.O#",
///     "#......O.#",
///     "#.OO..O.O#",
///     "#..O@..O.#",
///     "#O#..O...#",
///     "#O..O..O.#",
///     "#.OO.O.OO#",
///     "#....O...#",
///     "##########"
/// ].iter().map(|s| s.to_string()).collect(),
/// vec![
///     "<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^",
///     "vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v",
///     "><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<",
///     "<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^",
///     "^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><",
///     "^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^",
///     ">^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^",
///     "<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>",
///     "^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>",
///     "v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
/// ].iter().map(|s| s.to_string()).collect()];
/// assert_eq!(day15::puzzle_a(&vec1), 10092);
/// ```
pub fn puzzle_a(string_list: &Vec<Vec<String>>) -> usize {
    let (warehouse, mut robot, mut boxes) = parse_warehouse(string_list.first().unwrap());
    let directions = parse_moves(string_list.last().unwrap());
    for dir in directions {
        (robot, boxes) = step(&warehouse, robot, &boxes, dir);
    }
    return boxes.iter().map(|coord| coord.x + coord.y * 100).sum();
}

// We need to get: Map, Robot location (@), Box (O)
fn parse_double_warehouse(strings: &Vec<String>) -> (Map, Coord, Vec<(Coord, Coord)>) {
    let mut robot_coord: Coord = Coord::new(0, 0);
    let mut values = vec![];
    let height = strings.len();
    let width = strings.first().unwrap().len() * 2;
    let mut boxes = vec![];

    for (y, line) in strings.into_iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    values.push(Terrain::Wall);
                    values.push(Terrain::Wall);
                }
                '.' => {
                    values.push(Terrain::Empty);
                    values.push(Terrain::Empty);
                }
                '@' => {
                    robot_coord = Coord::new(2 * x, y);
                    values.push(Terrain::Empty);
                    values.push(Terrain::Empty);
                }
                'O' => {
                    values.push(Terrain::Empty);
                    values.push(Terrain::Empty);
                    boxes.push((Coord::new(2 * x, y), Coord::new(2 * x + 1, y)));
                }
                _ => panic!("Unknown character - Terrain {}", c),
            }
        }
    }

    let grid = Map::new(width, height, values);
    return (grid, robot_coord, boxes);
}

fn step_double(
    map: &Map,
    robot: Coord,
    boxes: &Vec<(Coord, Coord)>,
    direction: Direction,
) -> (Coord, Vec<(Coord, Coord)>) {
    let mut new_robot = robot.clone();
    let mut new_boxes = vec![];

    let potential_new_coord = map.get_coordinate_by_direction(robot, direction).unwrap();
    info!("Checking if can go to {}", potential_new_coord);
    let potential_value = map.get_value(potential_new_coord).unwrap();
    match potential_value {
        Terrain::Wall => {
            // No moves to do, moving into a wall
            new_boxes = boxes.clone();
        }
        Terrain::Empty => {
            // potentially valid move!
            // check if there is a box here
            // If direction is east or west, everything is easy!
            let complex_trace = direction == Direction::SOUTH || direction == Direction::NORTH;
            let mut boxes_to_move: HashSet<(Coord, Coord)> = HashSet::new();
            let matching_box = boxes.iter().find(|&&(left, right)| {
                left == potential_new_coord || right == potential_new_coord
            });
            if let Some(box_value) = matching_box {
                // find all boxes in a row / column from here, until we hit an emtpy space, or a wall.
                boxes_to_move.insert(*box_value);
                let mut locations_to_check: VecDeque<GridCoordinate> = VecDeque::new();
                if complex_trace {
                    locations_to_check.push_back(box_value.0);
                    locations_to_check.push_back(box_value.1);
                } else {
                    if direction == Direction::EAST {
                        locations_to_check.push_back(box_value.1);
                    } else {
                        locations_to_check.push_back(box_value.0);
                    }
                }
                let mut train_value;
                let mut good_to_move = true;
                let mut checked_boxes = HashSet::new();
                while let Some(check_coord) = locations_to_check.pop_front() {
                    if checked_boxes.contains(&check_coord) {
                        continue;
                    }
                    checked_boxes.insert(check_coord);
                    let train_coord = map
                        .get_coordinate_by_direction(check_coord, direction)
                        .unwrap();
                    train_value = map.get_value(train_coord).unwrap();
                    match train_value {
                        Terrain::Wall => {
                            // Hit a wall
                            good_to_move = false;
                            break;
                        }
                        Terrain::Empty => {
                            // check if we have a box
                            let train_box = boxes.iter().find(|&&(left, right)| {
                                left == train_coord || right == train_coord
                            });
                            if let Some((left, right)) = train_box {
                                boxes_to_move.insert((*left, *right));
                                if complex_trace {
                                    locations_to_check.push_back(*left);
                                    locations_to_check.push_back(*right);
                                } else {
                                    if direction == Direction::EAST {
                                        locations_to_check.push_back(*right);
                                    } else {
                                        locations_to_check.push_back(*left);
                                    }
                                }
                            } else {
                                // No box, we are good to move here!
                                continue;
                            }
                        }
                    };
                }
                if good_to_move {
                    new_boxes.extend(boxes.iter().filter(|&x| !boxes_to_move.contains(x)));
                    for (cur_box_left, cur_box_right) in boxes_to_move {
                        new_boxes.push((
                            map.get_coordinate_by_direction(cur_box_left, direction)
                                .unwrap(),
                            map.get_coordinate_by_direction(cur_box_right, direction)
                                .unwrap(),
                        ));
                    }
                    new_robot = potential_new_coord;
                } else {
                    info!("Invalid move case");
                    // Invalid move
                    new_boxes = boxes.clone();
                    new_robot = robot.clone();
                }
            } else {
                info!("Just move");
                // No box, just move
                new_boxes = boxes.clone();
                new_robot = potential_new_coord;
            }
        }
    }

    info!("Robot is now at: {}", new_robot);
    return (new_robot, new_boxes);
}

/// Find all GPS in double space move
/// ```
/// let vec1: Vec<Vec<String>> = vec![vec![
///     "##########",
///     "#..O..O.O#",
///     "#......O.#",
///     "#.OO..O.O#",
///     "#..O@..O.#",
///     "#O#..O...#",
///     "#O..O..O.#",
///     "#.OO.O.OO#",
///     "#....O...#",
///     "##########"
/// ].iter().map(|s| s.to_string()).collect(),
/// vec![
///     "<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^",
///     "vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v",
///     "><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<",
///     "<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^",
///     "^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><",
///     "^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^",
///     ">^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^",
///     "<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>",
///     "^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>",
///     "v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
/// ].iter().map(|s| s.to_string()).collect()];
/// assert_eq!(day15::puzzle_b(&vec1), 9021);
/// ```
pub fn puzzle_b(string_list: &Vec<Vec<String>>) -> usize {
    let (warehouse, mut robot, mut boxes) = parse_double_warehouse(string_list.first().unwrap());
    let directions = parse_moves(string_list.last().unwrap());
    print_map(&warehouse, &robot, &boxes);
    for dir in directions {
        (robot, boxes) = step_double(&warehouse, robot, &boxes, dir);
        print_map(&warehouse, &robot, &boxes);
    }
    return boxes.iter().map(|(left, _)| left.x + left.y * 100).sum();
}

fn print_map(warehouse: &Map, robot: &Coord, boxes: &Vec<(Coord, Coord)>) {
    let mut values = vec!['.'; warehouse.get_height() * warehouse.get_width()];
    let width = warehouse.get_width();
    values[robot.x + robot.y * width] = '@';
    for (left, right) in boxes {
        values[left.x + left.y * width] = '[';
        values[right.x + right.y * width] = ']';
    }
    for coord in warehouse.coord_iter() {
        let v = warehouse.get_value(coord).unwrap();
        if v == Terrain::Wall {
            values[coord.x + coord.y * width] = '#';
        }
    }
    let mut lines = vec![];
    for (x, c) in values.into_iter().enumerate() {
        if x % width == 0 {
            lines.push(vec![]);
        }
        lines.last_mut().unwrap().push(c);
    }
    for line in lines {
        let str: String = line.into_iter().collect();
        info!("{}", str);
    }
    info!("----");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_map() {
        let input = vec![
            "########", "#..O.O.#", "##@.O..#", "#...O..#", "#.#.O..#", "#...O..#", "#......#",
            "########",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
        let (warehouse, mut robot, mut boxes) = parse_warehouse(&input);
        let input = vec!["<^^>>>vv<v>>v<<"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        let directions = parse_moves(&input);
        println!("Robot {}", robot);
        for dir in directions {
            (robot, boxes) = step(&warehouse, robot, &boxes, dir);
            println!("{} - {:?}", dir, robot);
        }
        let total: usize = boxes.iter().map(|coord| coord.x + coord.y * 100).sum();
        assert_eq!(total, 2028);
    }

    #[test]
    fn test_b_small() {
        let input = vec![
            "#######", "#...#.#", "#.....#", "#..OO@#", "#..O..#", "#.....#", "#######",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
        let (warehouse, mut robot, mut boxes) = parse_double_warehouse(&input);
        let input = vec!["<vv<<^^<<^^"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        let directions = parse_moves(&input);
        println!("{:?}, {:?}", robot, boxes);
        for dir in directions {
            println!("{}", dir);
            (robot, boxes) = step_double(&warehouse, robot, &boxes, dir);
            println!("{:?}, {:?}", robot, boxes);
        }
        let total: usize = boxes.iter().map(|(left, _)| left.x + left.y * 100).sum();
        assert_eq!(total, 105 + 207 + 306);
    }

    #[test]
    fn test_double_step_into_wall() {
        /*
        ####################
        ##....[]....[]..[]##
        ##............[]..##
        ##..[][]....[]..[]##
        ##...[]...[]..[]..##
        ##[]##....[]......##
        ##[][]@.......[]..##
        ##.....[]..[].[][]##
        ##........[]......##
        ####################

        ######
        #OO@.#
        ######
        <
                 */
        let input = vec!["######", "#OO@.#", "######"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        let (warehouse, mut robot, mut boxes) = parse_double_warehouse(&input);
        let input = vec!["<"].into_iter().map(|s| s.to_string()).collect();
        let directions = parse_moves(&input);
        for dir in directions {
            (robot, boxes) = step_double(&warehouse, robot, &boxes, dir);
        }
        assert_eq!(robot, Coord::new(6, 1));
    }
}
