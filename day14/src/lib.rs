extern crate filelib;

pub use filelib::load_no_blanks;
use gridlib::GridCoordinateInf;
use log::info;
use mathlib::modulusi64;

type Number = i64;
type Coord = GridCoordinateInf<Number>;

fn parse_robots(string_list: &Vec<String>) -> Vec<(Coord, Coord)> {
    let mut robots = vec![];
    for line in string_list {
        let (p_rest, v_commas) = line.split_once(" v=").unwrap();
        let (_, p_commas) = p_rest.split_once("p=").unwrap();
        let (p_x_s, p_y_s) = p_commas.split_once(",").unwrap();
        let (v_x_s, v_y_s) = v_commas.split_once(",").unwrap();
        let p_x = p_x_s.parse().unwrap();
        let p_y = p_y_s.parse().unwrap();
        let v_x = v_x_s.parse().unwrap();
        let v_y = v_y_s.parse().unwrap();
        let pos = Coord::new(p_x, p_y);
        let vel = Coord::new(v_x, v_y);
        robots.push((pos, vel));
    }
    return robots;
}

fn find_pos(pos: Coord, vec: Coord, width: usize, height: usize, seconds_elapsed: Number) -> Coord {
    // We teleport across, so we can just find out where we are, then take x % width, y % height.

    let x = modulusi64(pos.x + seconds_elapsed * vec.x, width as Number);
    let y = modulusi64(pos.y + seconds_elapsed * vec.y, height as Number);

    let new_pos = Coord::new(x, y);
    info!("pos {} -> {}", pos, new_pos);
    return new_pos;
}

fn sort_to_quadrants(robots: Vec<Coord>, width: usize, height: usize) -> Vec<usize> {
    let mut top_left = 0;
    let mut top_right = 0;
    let mut bottom_left = 0;
    let mut bottom_right = 0;

    // 11 wide, 7 tall
    // left 0-4, right 6-10
    // top 0-2, bottom 4-6
    let max_left = width / 2 - 1;
    let min_right = width / 2 + 1;
    let max_top = height / 2 - 1;
    let min_bottom = height / 2 + 1;

    for robot in robots {
        let is_left = robot.x <= max_left as Number;
        let is_right = robot.x >= min_right as Number;
        let is_top = robot.y <= max_top as Number;
        let is_bottom = robot.y >= min_bottom as Number;

        if is_left {
            if is_top {
                top_left += 1;
            } else if is_bottom {
                bottom_left += 1;
            }
        } else if is_right {
            if is_top {
                top_right += 1;
            } else if is_bottom {
                bottom_right += 1;
            }
        }
    }

    return vec![top_left, top_right, bottom_left, bottom_right];
}

/// Calculate the safety factor
/// ```
/// let vec1: Vec<String> = vec![
///     "p=0,4 v=3,-3",
///     "p=6,3 v=-1,-3",
///     "p=10,3 v=-1,2",
///     "p=2,0 v=2,-1",
///     "p=0,0 v=1,3",
///     "p=3,0 v=-2,-2",
///     "p=7,6 v=-1,-3",
///     "p=3,0 v=-1,-2",
///     "p=9,3 v=2,3",
///     "p=7,3 v=-1,2",
///     "p=2,4 v=2,-3",
///     "p=9,5 v=-3,-3"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day14::puzzle_a(&vec1, 7, 11), 12);
/// ```
pub fn puzzle_a(string_list: &Vec<String>, height: usize, width: usize) -> usize {
    let seconds_elapsed = 100;
    let values = parse_robots(string_list);
    let final_locations = values
        .into_iter()
        .map(|(pos, vec)| find_pos(pos, vec, width, height, seconds_elapsed))
        .collect();
    return sort_to_quadrants(final_locations, width, height)
        .into_iter()
        .product();
}

// guessing that the tree needs a lot of robots in unique positions?
fn get_num_unique(positions: Vec<Coord>) -> usize {
    let mut robots = positions.clone();
    robots.sort();
    robots.dedup();
    return robots.len();
}

/// Find an iteration that looks like a Christmas tree??? How do I define that???
/// ```
/// let vec1: Vec<String> = vec![
///     "p=0,4 v=3,-3",
///     "p=6,3 v=-1,-3",
///     "p=10,3 v=-1,2",
///     "p=2,0 v=2,-1",
///     "p=0,0 v=1,3",
///     "p=3,0 v=-2,-2",
///     "p=7,6 v=-1,-3",
///     "p=3,0 v=-1,-2",
///     "p=9,3 v=2,3",
///     "p=7,3 v=-1,2",
///     "p=2,4 v=2,-3",
///     "p=9,5 v=-3,-3"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day14::puzzle_b(&vec1, 7, 11), 1);
/// ```
pub fn puzzle_b(string_list: &Vec<String>, height: usize, width: usize) -> usize {
    let values = parse_robots(string_list);

    let mut max_value = 0;
    let mut max_i: usize = 0;

    // Okay lets think about this
    // 103 * 101 = 10403 is the max number of possible spaces, things will definitely loop by then, so thats an upper bound.
    for i in 1..=(width * height) {
        let final_locations = values
            .clone()
            .into_iter()
            .map(|(pos, vec)| find_pos(pos, vec, width, height, i as Number))
            .collect();
        let num_pos = get_num_unique(final_locations);
        if num_pos > max_value {
            max_value = num_pos;
            max_i = i;
        }
    }
    return max_i;
}
