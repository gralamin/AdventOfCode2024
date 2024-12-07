extern crate filelib;

pub use filelib::load_no_blanks;
use log::info;

type Number = i64;

fn parse_equations(input: &Vec<String>) -> Vec<(Number, Vec<Number>)> {
    let mut result = vec![];
    for line in input.into_iter() {
        let (test, others) = line.split_once(":").unwrap();
        let parsed_test = test.parse().unwrap();
        let mut parsed_valued: Vec<Number> = vec![];
        for v in others.trim().split(" ") {
            let z = v.trim().parse().unwrap();
            parsed_valued.push(z);
        }

        result.push((parsed_test, parsed_valued));
    }
    return result;
}

fn line_is_solvable(test_value: Number, numbers: &Vec<Number>) -> bool {
    if numbers.len() <= 1 {
        return test_value == numbers[0];
    }
    info!("Solving: {:?}", numbers);
    let other_values = numbers[1..].to_vec();
    return recurse_solve(test_value, &other_values, numbers[0]);
}

fn recurse_solve(target: Number, numbers: &Vec<Number>, current: Number) -> bool {
    if current > target {
        return false;
    }
    if numbers.is_empty() {
        if current == target {
            info!("Solved!");
        }
        return current == target;
    }
    let v = numbers[0];
    let rest = numbers[1..].to_vec();
    return recurse_solve(target, &rest, current + v) || recurse_solve(target, &rest, current * v);
}

/// Evaluate left ot right, and only + or *. Filter out unsolvable, sum the test values of others.
/// ```
/// let vec1: Vec<String> = vec![
///     "190: 10 19",
///     "3267: 81 40 27",
///     "83: 17 5",
///     "156: 15 6",
///     "7290: 6 8 6 15",
///     "161011: 16 10 13",
///     "192: 17 8 14",
///     "21037: 9 7 18 13",
///     "292: 11 6 16 20"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day07::puzzle_a(&vec1), 3749);
/// ```
pub fn puzzle_a(string_list: &Vec<String>) -> Number {
    let lines = parse_equations(string_list);
    return lines
        .into_iter()
        .filter(|(test, others)| line_is_solvable(*test, others))
        .map(|(test, _)| test)
        .sum();
}

/// Foo
/// ```
/// let vec1: Vec<String> = vec![
///     "foo"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day07::puzzle_b(&vec1), 0);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> u32 {
    return 0;
}
