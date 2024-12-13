extern crate filelib;

pub use filelib::{load, split_lines_by_blanks};
use gridlib::GridCoordinateInf;
use log::info;

type Number = i64;
type Coord = GridCoordinateInf<Number>;

fn parse_prize_machines(string_list: &Vec<Vec<String>>) -> Vec<(Coord, Coord, Coord)> {
    let mut result = vec![];
    for prize in string_list {
        let line_a: &String = prize.first().unwrap();
        let line_b: &String = prize.iter().nth(1).unwrap();
        let line_prize: &String = prize.last().unwrap();
        let a = split_button_line(line_a);
        let b = split_button_line(line_b);
        let prize = split_prize_line(line_prize);
        result.push((a, b, prize));
    }
    return result;
}

fn split_button_line(button_line: &String) -> Coord {
    let (rest, yplus_s) = button_line.split_once(", ").unwrap();
    let (_, xvalue_s) = rest.split_once("X+").unwrap();
    let (_, yvalue_s) = yplus_s.split_once("Y+").unwrap();
    let x = xvalue_s.parse().unwrap();
    let y = yvalue_s.parse().unwrap();
    return Coord::new(x, y);
}

fn split_prize_line(button_line: &String) -> Coord {
    let (rest, yequal_s) = button_line.split_once(", ").unwrap();
    let (_, xvalue_s) = rest.split_once("X=").unwrap();
    let (_, yvalue_s) = yequal_s.split_once("Y=").unwrap();
    let x = xvalue_s.parse().unwrap();
    let y = yvalue_s.parse().unwrap();
    return Coord::new(x, y);
}

fn find_cheapest_button_presses(
    a_button: Coord,
    b_button: Coord,
    prize: Coord,
    max_press: Number,
    a_price: Number,
    b_price: Number,
) -> Option<u64> {
    // We want to treat this as a linear algebra problem
    // to find solutions we need to solve
    /*
       | ax, bx | * |a_press| = |prize_x|
       | ay, by |   |b_press|   |prize_y|

       This requires inverting the matrix to multiply it over to the other side so we can solve for presses. We remember this is:
       1 / determinent of M * adjoint of M
       Determinent here is ax * by - bx * ay
       adjoint is |by, -bx|
                  |-ay, ax|

       That means the matrix multiplicaiton here is:
       (by * prize_x - bx * prize_y) / determinent = a_press
       (ax * prize_y - ay * prize_x) / determinent = b_press
    */
    info!("Finding solution for {:?}", prize);
    let determinent = a_button.x * b_button.y - b_button.x * a_button.y;
    if determinent == 0 {
        return None;
    }
    let a_press_count = (b_button.y * prize.x - b_button.x * prize.y) / determinent;
    if a_press_count < 0 || a_press_count > max_press {
        return None;
    }
    let b_press_count = (a_button.x * prize.y - a_button.y * prize.x) / determinent;
    if b_press_count < 0 || b_press_count > max_press {
        return None;
    }
    let cost = a_press_count * a_price + b_press_count * b_price;
    let check_works_x = a_button.x * a_press_count + b_button.x * b_press_count;
    let check_works_y = a_button.y * a_press_count + b_button.y * b_press_count;
    if check_works_x == prize.x && check_works_y == prize.y {
        info!("Found {:?}", cost);
        return Some(cost as u64);
    }
    // Otherwise the answer is not an integer
    info!("Solution not positive integer");
    return None;
}

/// Try to find each prize within 100 presses using A and B buttons
/// ```
/// let vec1: Vec<Vec<String>> = vec![vec![
///     "Button A: X+94, Y+34",
///     "Button B: X+22, Y+67",
///     "Prize: X=8400, Y=5400",
///     ].iter().map(|s| s.to_string()).collect(), vec![
///     "Button A: X+26, Y+66",
///     "Button B: X+67, Y+21",
///     "Prize: X=12748, Y=12176",
///     ].iter().map(|s| s.to_string()).collect(), vec![
///     "Button A: X+17, Y+86",
///     "Button B: X+84, Y+37",
///     "Prize: X=7870, Y=6450",
///     ].iter().map(|s| s.to_string()).collect(), vec![
///     "Button A: X+69, Y+23",
///     "Button B: X+27, Y+71",
///     "Prize: X=18641, Y=10279"
/// ].iter().map(|s| s.to_string()).collect()];
/// assert_eq!(day13::puzzle_a(&vec1), 480);
/// ```
pub fn puzzle_a(string_list: &Vec<Vec<String>>) -> u64 {
    let a_cost = 3;
    let b_cost = 1;
    let max_press = 100;
    let mut total_cost = 0;
    let prizes = parse_prize_machines(string_list);
    for (a_button, b_button, prize) in prizes {
        let result =
            find_cheapest_button_presses(a_button, b_button, prize, max_press, a_cost, b_cost);
        if let Some(cost) = result {
            total_cost += cost;
        }
    }
    return total_cost;
}

/// As above, but infinite presses and each prize is an absurd distance away
/// ```
/// let vec1: Vec<Vec<String>> = vec![vec![
///     "Button A: X+94, Y+34",
///     "Button B: X+22, Y+67",
///     "Prize: X=8400, Y=5400",
///     ].iter().map(|s| s.to_string()).collect(), vec![
///     "Button A: X+26, Y+66",
///     "Button B: X+67, Y+21",
///     "Prize: X=12748, Y=12176",
///     ].iter().map(|s| s.to_string()).collect(), vec![
///     "Button A: X+17, Y+86",
///     "Button B: X+84, Y+37",
///     "Prize: X=7870, Y=6450",
///     ].iter().map(|s| s.to_string()).collect(), vec![
///     "Button A: X+69, Y+23",
///     "Button B: X+27, Y+71",
///     "Prize: X=18641, Y=10279"
/// ].iter().map(|s| s.to_string()).collect()];
/// assert_eq!(day13::puzzle_b(&vec1), 875318608908);
/// ```
pub fn puzzle_b(string_list: &Vec<Vec<String>>) -> u64 {
    let adjustment = 10000000000000;
    let a_cost = 3;
    let b_cost = 1;
    let max_press = Number::MAX;
    let mut total_cost: u64 = 0;
    let prizes = parse_prize_machines(string_list);
    for (a_button, b_button, prize) in prizes {
        let true_prize = Coord::new(adjustment + prize.x, adjustment + prize.y);
        let result =
            find_cheapest_button_presses(a_button, b_button, true_prize, max_press, a_cost, b_cost);
        if let Some(cost) = result {
            total_cost += cost;
        }
    }
    return total_cost;
}
