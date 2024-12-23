extern crate filelib;

use std::collections::HashMap;

pub use filelib::load_no_blanks;

type Number = u64;

fn parse_stones(string_list: &Vec<String>) -> Vec<Number> {
    let mut results = vec![];
    // one line
    let line = string_list.first().unwrap();
    for a in line.split(" ") {
        let v = a.parse().unwrap();
        results.push(v);
    }
    return results;
}

fn count_digits(num: Number) -> u32 {
    return num.checked_ilog10().unwrap_or(0) + 1;
}

// Rules: 0 -> 1
// even digits -> Split in two, halving the number (so 1000 -> 10 and 0. 9321 becomes 93 21)
// Otherwise number *2024
// order perserved
fn blink_once(stone_state: Vec<Number>) -> Vec<Number> {
    let mut next_state = vec![];

    for v in stone_state {
        let digit_count = count_digits(v);
        if v == 0 {
            next_state.push(1);
        } else if digit_count % 2 == 0 {
            // 10 -> 2 digits -> divide by 10 to split -> pow 1
            // 1000 -> 4 digits -> divide by 100 to split -> pow 2
            // 100000 -> 6 digits -> divide by 1000 to split -> pow 3
            let base: Number = 10;
            let splitter = base.pow(digit_count / 2);
            let first = v / splitter;
            let second = v % splitter;
            next_state.push(first);
            next_state.push(second);
        } else {
            next_state.push(v * 2024);
        }
    }

    return next_state;
}

/// Blink 25 times and count stones
/// ```
/// let vec1: Vec<String> = vec![
///     "125 17"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day11::puzzle_a(&vec1), 55312);
/// ```
pub fn puzzle_a(string_list: &Vec<String>) -> usize {
    let mut cur_stone_list = parse_stones(string_list);
    for _ in 0..25 {
        cur_stone_list = blink_once(cur_stone_list);
    }
    return cur_stone_list.len();
}

// Rules: 0 -> 1
// even digits -> Split in two, halving the number (so 1000 -> 10 and 0. 9321 becomes 93 21)
// Otherwise number *2024
// order perserved
// We don't actually care about the order though, so... HashMap
fn blink_faster(stone_state: &HashMap<Number, usize>) -> HashMap<Number, usize> {
    let mut next_state = HashMap::new();

    for (&key, &count) in stone_state {
        let digit_count = count_digits(key);
        if key == 0 {
            *next_state.entry(1).or_insert(0) += count;
        } else if digit_count % 2 == 0 {
            // 10 -> 2 digits -> divide by 10 to split -> pow 1
            // 1000 -> 4 digits -> divide by 100 to split -> pow 2
            // 100000 -> 6 digits -> divide by 1000 to split -> pow 3
            let base: Number = 10;
            let splitter = base.pow(digit_count / 2);
            let first = key / splitter;
            let second = key % splitter;
            *next_state.entry(first).or_insert(0) += count;
            *next_state.entry(second).or_insert(0) += count;
        } else {
            *next_state.entry(key * 2024).or_insert(0) += count;
        }
    }

    return next_state;
}

/// Blink 25 times and count stones
/// ```
/// let vec1: Vec<String> = vec![
///     "125 17"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day11::puzzle_b(&vec1), 65601038650482);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> usize {
    let cur_stone_list = parse_stones(string_list);
    let mut hash_state: HashMap<Number, usize> = HashMap::new();
    for v in cur_stone_list {
        *hash_state.entry(v).or_insert(0) += 1;
    }

    for _ in 0..75 {
        hash_state = blink_faster(&hash_state);
    }
    return hash_state.values().sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blink_once() {
        let input = vec![0, 1, 10, 99, 999];
        let v = blink_once(input);
        assert_eq!(v, vec![1, 2024, 1, 0, 9, 9, 2021976]);
    }
}
