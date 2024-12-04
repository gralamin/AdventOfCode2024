extern crate filelib;

pub use filelib::load_no_blanks;
use gridlib::GridTraversable;
use gridlib::{Direction, Grid, GridCoordinate};
use log::info;

#[derive(PartialEq, Debug, Copy, Clone)]
enum XMASChar {
    X,
    M,
    A,
    S,
    End,
}

impl XMASChar {
    fn get_next_char(&self) -> XMASChar {
        return match self {
            XMASChar::X => XMASChar::M,
            XMASChar::M => XMASChar::A,
            XMASChar::A => XMASChar::S,
            XMASChar::S => XMASChar::End,
            XMASChar::End => XMASChar::End,
        };
    }

    fn from_char(c: char) -> Option<XMASChar> {
        return match c {
            'A' => Some(XMASChar::A),
            'M' => Some(XMASChar::M),
            'S' => Some(XMASChar::S),
            'X' => Some(XMASChar::X),
            _ => None,
        };
    }
}

fn parse(lines: &Vec<String>) -> Grid<XMASChar> {
    let height = lines.len();
    let width = lines.first().unwrap().len();
    let mut values: Vec<XMASChar> = vec![];
    for line in lines {
        for v in line.chars() {
            let new_value = XMASChar::from_char(v);
            match new_value {
                Some(x) => values.push(x),
                None => continue,
            }
        }
    }
    return Grid::new(width, height, values);
}

// Find all of a specific coordinates.
fn find_all_char(grid: &Grid<XMASChar>, matching: XMASChar) -> Vec<GridCoordinate> {
    let mut x_coords: Vec<GridCoordinate> = vec![];
    for coord in grid.coord_iter() {
        if let Some(v) = grid.get_value(coord) {
            if v == matching {
                x_coords.push(coord);
            }
        }
    }
    return x_coords;
}

// For each X, look in all directions for it for Ms, and add to a stack.
// We can then leave them to handle themselves as they come off the stack.
fn search(grid: &Grid<XMASChar>, x_coords: &Vec<GridCoordinate>) -> u32 {
    let mut search_at: Vec<(Vec<GridCoordinate>, Direction, GridCoordinate)> = vec![];
    let mut found = 0;
    for x_pos in x_coords {
        for direction in [
            Direction::NORTH,
            Direction::EAST,
            Direction::WEST,
            Direction::SOUTH,
            Direction::NORTHEAST,
            Direction::SOUTHEAST,
            Direction::NORTHWEST,
            Direction::SOUTHWEST,
        ] {
            // path, direction to keep searching in, current_position
            search_at.push((vec![*x_pos], direction, *x_pos));
        }
    }

    while let Some((path_so_far, direction, current_position)) = search_at.pop() {
        let current_char = grid.get_value(current_position);
        let next_char;
        next_char = match current_char {
            Some(x) => x.get_next_char(),
            None => continue,
        };
        if next_char == XMASChar::End {
            // we have hit a match!
            info!("Found path {:?}", path_so_far);
            found += 1;
            continue;
        }
        let next_coord = grid.get_coordinate_by_direction(current_position, direction);
        match next_coord {
            None => continue,
            Some(coord) => {
                let value_at_next = grid.get_value(coord);
                match value_at_next {
                    Some(x) => {
                        if x == next_char {
                            let mut new_path = path_so_far.clone();
                            new_path.push(coord);
                            search_at.push((new_path, direction, coord));
                        }
                    }
                    None => continue,
                }
            }
        }
    }
    return found;
}

/// Find all XMAS in a wordsearch, including overlaps. Can be backwards.
/// ```
/// let vec1: Vec<String> = vec![
///     "MMMSXXMASM",
///     "MSAMXMSMSA",
///     "AMXSXMAAMM",
///     "MSAMASMSMX",
///     "XMASAMXAMM",
///     "XXAMMXXAMA",
///     "SMSMSASXSS",
///     "SAXAMASAAA",
///     "MAMMMXMMMM",
///     "MXMXAXMASX"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day04::puzzle_a(&vec1), 18);
/// ```
pub fn puzzle_a(string_list: &Vec<String>) -> u32 {
    let grid = parse(string_list);
    let x_coords = find_all_char(&grid, XMASChar::X);
    return search(&grid, &x_coords);
}

// For each a, we just need to make sure we have two Ms, two Ss, and that any one set of opposite corners don't match.
fn xsearch(grid: &Grid<XMASChar>, a_coords: &Vec<GridCoordinate>) -> u32 {
    let mut result = 0;
    for a_coord in a_coords {
        let ne_coord;
        match grid.get_coordinate_by_direction(*a_coord, Direction::NORTHEAST) {
            Some(coord) => ne_coord = coord,
            None => continue,
        };
        let se_coord;
        match grid.get_coordinate_by_direction(*a_coord, Direction::SOUTHEAST) {
            Some(coord) => se_coord = coord,
            None => continue,
        };
        let nw_coord;
        match grid.get_coordinate_by_direction(*a_coord, Direction::NORTHWEST) {
            Some(coord) => nw_coord = coord,
            None => continue,
        };
        let sw_coord;
        match grid.get_coordinate_by_direction(*a_coord, Direction::SOUTHWEST) {
            Some(coord) => sw_coord = coord,
            None => continue,
        };
        let ne = grid.get_value(ne_coord).unwrap();
        let se = grid.get_value(se_coord).unwrap();
        let nw = grid.get_value(nw_coord).unwrap();
        let sw = grid.get_value(sw_coord).unwrap();
        let s_count = vec![ne, se, nw, sw]
            .into_iter()
            .filter(|&v| v == XMASChar::S)
            .collect::<Vec<_>>()
            .len();
        let m_count = vec![ne, se, nw, sw]
            .into_iter()
            .filter(|&v| v == XMASChar::M)
            .collect::<Vec<_>>()
            .len();
        if s_count != 2 || m_count != 2 {
            continue;
        }
        if ne == sw || se == nw {
            continue;
        }
        info!("Found X at A centered at {:?}", a_coord);
        result += 1;
    }
    return result;
}

/// Find all MAS in the shape of an X
/// ```
/// let vec1: Vec<String> = vec![
///     "MMMSXXMASM",
///     "MSAMXMSMSA",
///     "AMXSXMAAMM",
///     "MSAMASMSMX",
///     "XMASAMXAMM",
///     "XXAMMXXAMA",
///     "SMSMSASXSS",
///     "SAXAMASAAA",
///     "MAMMMXMMMM",
///     "MXMXAXMASX"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day04::puzzle_b(&vec1), 9);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> u32 {
    let grid = parse(string_list);
    let a_coords = find_all_char(&grid, XMASChar::A);
    return xsearch(&grid, &a_coords);
}
