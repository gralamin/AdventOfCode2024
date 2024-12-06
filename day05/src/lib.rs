extern crate filelib;

pub use filelib::{load, split_lines_by_blanks};
use std::cmp::Ordering;
use std::collections::HashSet;

use log::info;

fn parse_graph(string_list: &Vec<String>) -> HashSet<(i32, i32)> {
    info!("GraphParsing: {:?}", string_list);
    let mut result = HashSet::new();
    for s in string_list {
        let (i, j) = s.split_once("|").unwrap();
        let before = i.parse().unwrap();
        let after = j.parse().unwrap();
        result.insert((before, after));
    }
    return result;
}

fn parse_pages(string_list: &Vec<String>) -> Vec<Vec<i32>> {
    info!("PageParsing: {:?}", string_list);
    let list_of_list_of_values: Vec<Vec<&str>> = string_list
        .into_iter()
        .map(|x| x.split(",").collect::<Vec<&str>>())
        .collect();
    return list_of_list_of_values
        .iter()
        .map(|list| {
            list.into_iter()
                .map(|x| x.parse::<i32>().unwrap())
                .collect()
        })
        .collect();
}

fn page_valid(graph: &HashSet<(i32, i32)>, page: &Vec<i32>) -> bool {
    info!("Checking page {:?}", page);
    for i in 0..page.len() {
        let left = page[i];
        for j in i + 1..page.len() {
            let right = page[j];
            if graph.contains(&(right, left)) {
                info!("Found broken rule ({}|{})", right, left);
                return false;
            }
        }
    }
    return true;
}

/// Build hash graph and check ordering.
/// ```
/// let vec1: Vec<String> = vec![
///     "47|53",
///     "97|13",
///     "97|61",
///     "97|47",
///     "75|29",
///     "61|13",
///     "75|53",
///     "29|13",
///     "97|29",
///     "53|29",
///     "61|53",
///     "97|53",
///     "61|29",
///     "47|13",
///     "75|47",
///     "97|75",
///     "47|61",
///     "75|61",
///     "47|29",
///     "75|13",
///     "53|13",
///     "",
///     "75,47,61,53,29",
///     "97,61,53,29,13",
///     "75,29,13",
///     "75,97,47,61,53",
///     "61,13,29",
///     "97,13,75,29,47"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day05::puzzle_a(&vec1.join("\n")), 143);
/// ```
pub fn puzzle_a(string_list: &String) -> i32 {
    // groups[0] is depedency graph
    // groups[1] is pagesToProduce
    let groups = split_lines_by_blanks(string_list);
    let graph = parse_graph(&groups[0]);
    let pages = parse_pages(&groups[1]);
    return pages
        .into_iter()
        .filter(|x| page_valid(&graph, x))
        .map(|x| x[x.len() / 2])
        .sum();
}

fn page_reorder(graph: &HashSet<(i32, i32)>, page: &Vec<i32>) -> Vec<i32> {
    info!("reordering page {:?}", page);
    let mut result = page.clone();
    result.sort_by(|&a, &b| {
        if graph.contains(&(a, b)) {
            Ordering::Less
        } else if graph.contains(&(b, a)) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });
    info!("reordered page {:?}", page);
    return result;
}

/// sort the wrong pages
/// ```
/// let vec1: Vec<String> = vec![
///     "47|53",
///     "97|13",
///     "97|61",
///     "97|47",
///     "75|29",
///     "61|13",
///     "75|53",
///     "29|13",
///     "97|29",
///     "53|29",
///     "61|53",
///     "97|53",
///     "61|29",
///     "47|13",
///     "75|47",
///     "97|75",
///     "47|61",
///     "75|61",
///     "47|29",
///     "75|13",
///     "53|13",
///     "",
///     "75,47,61,53,29",
///     "97,61,53,29,13",
///     "75,29,13",
///     "75,97,47,61,53",
///     "61,13,29",
///     "97,13,75,29,47"
/// ].iter().map(|s| s.to_string()).collect();

/// assert_eq!(day05::puzzle_b(&vec1.join("\n")), 123);
/// ```
pub fn puzzle_b(string_list: &String) -> i32 {
    let groups = split_lines_by_blanks(string_list);
    let graph = parse_graph(&groups[0]);
    let pages = parse_pages(&groups[1]);
    return pages
        .into_iter()
        .filter(|x| !page_valid(&graph, x))
        .map(|x| page_reorder(&graph, &x))
        .map(|x| x[x.len() / 2])
        .sum();
}
