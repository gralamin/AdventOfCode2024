extern crate filelib;

use std::collections::{HashMap, HashSet, VecDeque};

pub use filelib::{load, split_lines_by_blanks};
use log::info;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum TowelColor {
    White,
    Blue,
    Black,
    Red,
    Green,
}

fn parse_towels(input: &String) -> Vec<Vec<TowelColor>> {
    let mut result = vec![];
    for towel in input.split(", ") {
        let mut current = vec![];
        for c in towel.chars() {
            current.push(match c {
                'w' => TowelColor::White,
                'u' => TowelColor::Blue,
                'b' => TowelColor::Black,
                'r' => TowelColor::Red,
                'g' => TowelColor::Green,
                _ => panic!("Unknown color in towels {}", c),
            });
        }
        result.push(current);
    }
    return result;
}

fn parse_patterns(input: &Vec<String>) -> Vec<Vec<TowelColor>> {
    let mut result = vec![];
    for pattern in input {
        let mut current = vec![];
        for c in pattern.chars() {
            current.push(match c {
                'w' => TowelColor::White,
                'u' => TowelColor::Blue,
                'b' => TowelColor::Black,
                'r' => TowelColor::Red,
                'g' => TowelColor::Green,
                _ => panic!("Unknown color in towels {}", c),
            });
        }
        result.push(current);
    }
    return result;
}

fn is_possible(towels: &Vec<Vec<TowelColor>>, pattern: &Vec<TowelColor>) -> bool {
    // we can treat this as an explorable space
    let mut visited: HashSet<Vec<TowelColor>> = HashSet::new();
    let mut queue: VecDeque<(Vec<TowelColor>, Vec<Vec<TowelColor>>)> = VecDeque::new();

    queue.push_back((pattern.clone(), vec![]));
    while let Some((cur_to_solve, cur_solution)) = queue.pop_front() {
        if cur_to_solve.len() == 0 {
            // Solved!
            info!("Solution found for {:?} - {:?}", towels, cur_solution);
            return true;
        }
        if visited.contains(&cur_to_solve) {
            continue;
        }
        visited.insert(cur_to_solve.clone());

        for potential_solution in towels {
            if potential_solution.len() > cur_to_solve.len() {
                continue;
            }
            let mut left = cur_to_solve.clone();
            let right = left.split_off(potential_solution.len());
            if left != *potential_solution {
                continue;
            }
            let mut next_solution = cur_solution.clone();
            next_solution.push(potential_solution.clone());
            queue.push_back((right, next_solution));
        }
    }
    return false;
}

/// Arrange some towels, get possible to solve
/// ```
/// let vec1: Vec<Vec<String>> = vec![vec![
///     "r, wr, b, g, bwu, rb, gb, br"
/// ].iter().map(|s| s.to_string()).collect(), vec![
///     "brwrr",
///     "bggr",
///     "gbbr",
///     "rrbgbr",
///     "ubwu",
///     "bwurrg",
///     "brgr",
///     "bbrgwb"
/// ].iter().map(|s| s.to_string()).collect()];
/// assert_eq!(day19::puzzle_a(&vec1), 6);
/// ```
pub fn puzzle_a(string_list: &Vec<Vec<String>>) -> usize {
    let towels = parse_towels(string_list.first().unwrap().first().unwrap());
    let patterns = parse_patterns(string_list.last().unwrap());
    let possible: Vec<Vec<TowelColor>> = patterns
        .into_iter()
        .filter(|pattern| is_possible(&towels, pattern))
        .collect();
    return possible.len();
}

// We can't actually get all possible, we run out of memory :(
// Get the count instead.
type Cache = HashMap<Vec<TowelColor>, usize>;
type ListOfColorCombos = Vec<Vec<TowelColor>>;

fn get_all_possible(
    towels: &ListOfColorCombos,
    pattern: &Vec<TowelColor>,
    solution_so_far: &ListOfColorCombos,
    cache: &mut Cache,
) -> usize {
    // we can treat this as an explorable space
    let mut valid_results = 0;

    if let Some(prev) = cache.get(pattern) {
        return prev.clone();
    }

    if pattern.len() == 0 {
        // valid solution
        return 1;
    }

    for potential_solution in towels {
        if !pattern.starts_with(&potential_solution) {
            continue;
        }
        let (left, rest) = pattern.split_at(potential_solution.len());
        let mut next_solution = solution_so_far.clone();
        next_solution.push(left.to_vec());
        let solved = get_all_possible(towels, &rest.to_vec(), &next_solution, cache);
        valid_results += solved;
    }
    *cache.entry(pattern.clone()).or_insert(0) += valid_results;
    return valid_results;
}

/// Provide all possible arrangements instead.
/// ```
/// let vec1: Vec<Vec<String>> = vec![vec![
///     "r, wr, b, g, bwu, rb, gb, br"
/// ].iter().map(|s| s.to_string()).collect(), vec![
///     "brwrr",
///     "bggr",
///     "gbbr",
///     "rrbgbr",
///     "ubwu",
///     "bwurrg",
///     "brgr",
///     "bbrgwb"
/// ].iter().map(|s| s.to_string()).collect()];
/// assert_eq!(day19::puzzle_b(&vec1), 16);
/// ```
pub fn puzzle_b(string_list: &Vec<Vec<String>>) -> usize {
    let towels = parse_towels(string_list.first().unwrap().first().unwrap());
    let patterns = parse_patterns(string_list.last().unwrap());
    let mut possible = 0;
    let mut cache = Cache::new();
    let default_solution = vec![];
    for pattern in patterns {
        if !is_possible(&towels, &pattern) {
            continue;
        }
        possible += get_all_possible(&towels, &pattern, &default_solution, &mut cache);
    }
    return possible;
}
