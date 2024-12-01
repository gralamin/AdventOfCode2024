extern crate filelib;

pub use filelib::load_no_blanks;
use std::collections::HashMap;

/// Get the sum of all first and last numbers in each line. If a single number appears in a line, count it for both.
/// ```
/// let vec1: Vec<String> = vec!["3 4", "4 3", "2 5", "1 3", "3 9", "3 3"].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day01::puzzle_a(&vec1), 11);
/// ```
pub fn puzzle_a(string_list: &Vec<String>) -> u32 {
    // split string_list into two lists of numbers
    let mut num1: Vec<u32> = vec![];
    let mut num2: Vec<u32> = vec![];
    for s in string_list {
        let (a_s, b_s) = s.split_once(" ").unwrap();
        let a = a_s.trim().parse::<u32>().unwrap();
        let b = b_s.trim().parse::<u32>().unwrap();
        num1.push(a);
        num2.push(b);
    }
    // Sort both lists
    num1.sort();
    num2.sort();

    let mut distance = 0;
    for (a, b) in num1.into_iter().zip(num2.into_iter()) {
        if a > b {
            distance += a - b;
        } else {
            distance += b - a;
        }
    }
    return distance;
}

/// Get the sum of all first and last numbers in each line, including number words. If a single number appears in a line, count it for both.
/// ```
/// let vec1: Vec<String> = vec!["3 4", "4 3", "2 5", "1 3", "3 9", "3 3"].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day01::puzzle_b(&vec1), 31);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> u32 {
    // split string_list into two lists of numbers
    let mut num1: Vec<u32> = vec![];
    let mut num2: Vec<u32> = vec![];
    for s in string_list {
        let (a_s, b_s) = s.split_once(" ").unwrap();
        let a = a_s.trim().parse::<u32>().unwrap();
        let b = b_s.trim().parse::<u32>().unwrap();
        num1.push(a);
        num2.push(b);
    }
    // Sort both lists
    num1.sort();
    num2.sort();

    let mut scores: HashMap<u32, u32> = HashMap::new();
    let mut total_score: u32 = 0;

    for n in num1 {
        match scores.get(&n) {
            Some(&score) => {
                total_score += n * score;
                continue;
            }
            _ => {}
        }
        let score: u32 = num2
            .iter()
            .filter(|&k| *k == n)
            .collect::<Vec<&u32>>()
            .len() as u32;
        total_score += n * score;
        scores.insert(n, score);
    }

    return total_score;
}
