extern crate filelib;

pub use filelib::load_no_blanks;
use log::info;
use regex::Regex;

fn extract_whole_muls(corruptInput: &Vec<String>) -> Vec<(i32, i32)> {
    let mut result = vec![];
    let extract_mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    for line in corruptInput {
        for capture in extract_mul_regex.captures_iter(line) {
            let (_, [a_s, b_s]) = capture.extract();
            info!("captured: {:?}, {:?}", a_s, b_s);
            let a: i32 = a_s.parse().unwrap();
            let b: i32 = b_s.parse().unwrap();
            result.push((a, b));
        }
    }
    return result;
}

/// Parse only things that match mul(x,y) exactly. Do not handle negative numbers. Add the results.
/// ```
/// let vec1: Vec<String> = vec![
///     "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day03::puzzle_a(&vec1), 161);
/// ```
pub fn puzzle_a(string_list: &Vec<String>) -> i32 {
    return extract_whole_muls(string_list)
        .into_iter()
        .map(|(x, y)| x * y)
        .sum();
}

/// Foo
/// ```
/// let vec1: Vec<String> = vec![
///     "foo"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day03::puzzle_b(&vec1), 0);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> i32 {
    return 0;
}
