extern crate filelib;

use std::cmp;
use std::collections::{HashMap, HashSet};

pub use filelib::load_no_blanks;
use log::info;

type SecretNumber = i64;

fn evolve_secret_number(n: SecretNumber) -> SecretNumber {
    let a = n * 64;
    let mut x = prune(mix(n, a));
    let b = x / 32;
    x = prune(mix(x, b));
    let c = x * 2048;
    return prune(mix(x, c));
}

fn mix(v: SecretNumber, into: SecretNumber) -> SecretNumber {
    return v ^ into;
}

fn prune(v: SecretNumber) -> SecretNumber {
    return v % 16777216;
}

fn evolve_secret_number_x_times(n: SecretNumber, x: usize) -> SecretNumber {
    let mut cur = n;
    for _ in 0..x {
        cur = evolve_secret_number(cur);
    }
    return cur;
}

/// Find the 2000th secret number and sum them
/// ```
/// let vec1: Vec<String> = vec![
///     "1",
///     "10",
///     "100",
///     "2024"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day22::puzzle_a(&vec1), 37327623);
/// ```
pub fn puzzle_a(string_list: &Vec<String>) -> SecretNumber {
    let input: Vec<SecretNumber> = string_list
        .iter()
        .map(|x| x.parse::<SecretNumber>().unwrap())
        .collect();
    return input
        .into_iter()
        .map(|x| evolve_secret_number_x_times(x, 2000))
        .sum();
}

type Cache = HashSet<(SecretNumber, SecretNumber, SecretNumber, SecretNumber)>;

fn get_all_evolutions_in_x_times(n: SecretNumber, x: usize) -> Vec<SecretNumber> {
    let mut cur = n;
    let mut result = vec![];
    for _ in 0..x {
        cur = evolve_secret_number(cur);
        result.push(cur);
    }
    return result;
}

fn calc_monkey_data(
    initial_values: Vec<SecretNumber>,
    num_changes: usize,
) -> Vec<(Vec<SecretNumber>, Vec<SecretNumber>, Vec<SecretNumber>)> {
    let mut monkey_data: Vec<(Vec<SecretNumber>, Vec<SecretNumber>, Vec<SecretNumber>)> = vec![];

    // For each monkey
    for monkey_initial in initial_values {
        let mut last_value = monkey_initial;
        // Generate the next 2000 secret numbers, and get the first digit of each.
        let a = get_all_evolutions_in_x_times(last_value, num_changes);
        let b: Vec<SecretNumber> = a.iter().map(|x| x % 10).collect();
        let mut c = vec![];
        // And get the differences between those
        for i in b.iter() {
            let diff = i.clone() - last_value % 10;
            c.push(diff);
            last_value = i.clone();
        }
        monkey_data.push((a, b, c));
    }
    return monkey_data;
}

fn calc_runs(
    monkey_data: Vec<(Vec<SecretNumber>, Vec<SecretNumber>, Vec<SecretNumber>)>,
) -> Vec<HashMap<(SecretNumber, SecretNumber, SecretNumber, SecretNumber), SecretNumber>> {
    let mut result = vec![];
    for (_, prices, diffs) in monkey_data.iter() {
        let mut cur_result = HashMap::new();
        for i in 0..diffs.len() - 4 {
            let a = diffs[i];
            let b = diffs[i + 1];
            let c = diffs[i + 2];
            let d = diffs[i + 3];
            let price = prices[i + 3];
            let key = (a, b, c, d);
            if cur_result.contains_key(&key) {
                continue;
            }
            cur_result.insert(key, price);
        }
        result.push(cur_result);
    }
    return result;
}

/// Generate secret number 2001 -> 4000, and find the run of price differences in that, such that it maximizes the overall profit.
/// ```
/// let vec1: Vec<String> = vec![
///     "1",
///     "2",
///     "3",
///     "2024"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day22::puzzle_b(&vec1, 0, 2000), 23);
/// ```
/// Skip iterations is here because I misunderstood the question, lol.
pub fn puzzle_b(
    string_list: &Vec<String>,
    skip_iterations: usize,
    then_iterations: usize,
) -> SecretNumber {
    let input: Vec<SecretNumber> = string_list
        .iter()
        .map(|x| x.parse::<SecretNumber>().unwrap())
        .collect();
    let initial_values: Vec<SecretNumber> = input
        .into_iter()
        .map(|x| evolve_secret_number_x_times(x, skip_iterations))
        .collect();
    let mut cache = Cache::new();

    let monkey_data = calc_monkey_data(initial_values, then_iterations);
    let runs = calc_runs(monkey_data);
    let mut highest_sum = 0;
    let default = 0;
    for monkey_runs in runs.iter() {
        for (key, _) in monkey_runs.iter() {
            if cache.contains(key) {
                continue;
            }

            let mut cur_sum = 0;
            for other_data in runs.iter() {
                let p: &SecretNumber = other_data.get(key).or(Some(&default)).unwrap();
                cur_sum += *p;
            }
            cache.insert(*key);

            info!("key: {:?}, gave sum {}", key, cur_sum);
            highest_sum = cmp::max(highest_sum, cur_sum);
        }
    }
    return highest_sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evolve() {
        let first = 123;
        let mut result = vec![first];
        let expected = vec![
            first, 15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484,
            7753432, 5908254,
        ];

        for _ in 0..10 {
            let last = result.iter().last().unwrap().clone();
            let next = evolve_secret_number(last);
            result.push(next);
        }

        assert_eq!(result, expected);
    }

    #[test]
    fn test_calc_monkey_data() {
        let first = 123;
        let result = calc_monkey_data(vec![first], 9);

        let expected_num_list = vec![
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
        ];

        let expected_digit_list = vec![0, 6, 5, 4, 4, 6, 4, 4, 2];
        let expected_diff = vec![-3, 6, -1, -1, 0, 2, -2, 0, -2];

        let (r_numlist, r_firstdigit, r_diff) = result.first().unwrap();

        assert_eq!(*r_numlist, expected_num_list);
        assert_eq!(*r_firstdigit, expected_digit_list);
        assert_eq!(*r_diff, expected_diff);
    }

    #[test]
    fn puzzle_b_single_input() {
        let vec1: Vec<String> = vec!["123"].iter().map(|s| s.to_string()).collect();
        assert_eq!(puzzle_b(&vec1, 0, 10), 6);
    }
}
