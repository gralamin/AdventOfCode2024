extern crate filelib;

pub use filelib::load_no_blanks;
use log::info;
use regex::Regex;

fn extract_whole_muls(corrupt_input: &Vec<String>) -> Vec<(i32, i32)> {
    let mut result = vec![];
    let extract_mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    for line in corrupt_input {
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

/// Split by dos and don't instructions, throw away the don'ts.
fn split_by_dos_and_donts(string_list: &Vec<String>) -> Vec<String> {
    let mut result = vec![];
    let mut in_do = true;
    let mut last_in_do = false;
    for s in string_list {
        let parts = s.split("don't()");
        for p in parts {
            last_in_do = false;
            if in_do {
                result.push(p.to_string());
                in_do = false;
                continue;
            }
            // Now definitely in a don't, split by do if any
            let mut first_dont = true;
            let do_parts = p.split("do()");
            for j in do_parts {
                if first_dont {
                    first_dont = false;
                    continue;
                }
                // these are all valid dos, push them in
                result.push(j.to_string());
                last_in_do = true;
            }
            // we are back in a don't definitely.
        }
        if last_in_do {
            last_in_do = false;
            in_do = true;
        }
    }
    return result;
}

/// As 1, Work with dos and don't. We start in do()
/// ```
/// let vec1: Vec<String> = vec![
///     "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day03::puzzle_b(&vec1), 48);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> i32 {
    let do_parts = split_by_dos_and_donts(string_list);
    return extract_whole_muls(&do_parts)
        .into_iter()
        .map(|(x, y)| x * y)
        .sum();
}
