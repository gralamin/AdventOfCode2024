extern crate filelib;

pub use filelib::load_no_blanks;
use log::info;

pub fn parse(string_list: &Vec<String>) -> Vec<Vec<u32>> {
    let mut result = vec![];
    for l in string_list {
        let mut cur = vec![];
        for s in l.split(" ") {
            if s.is_empty() {
                continue;
            }
            let v: u32 = s.parse().unwrap();
            cur.push(v);
        }
        result.push(cur);
    }
    return result;
}

/// is_safe if:
/// all increasing or decreasing
/// all adjacent levels differ by at least 1 and at most 3
fn is_safe(report: &Vec<u32>) -> bool {
    if report.len() < 2 {
        return true;
    }
    let mut last_level = report.first().unwrap();
    let mut handle_first = false;
    let mut is_increasing = false;
    let mut is_decreasing = false;

    info!("Current report {:?}", report);
    for level in report {
        if !handle_first {
            handle_first = true;
            continue;
        }
        info!("handling last {:?} and now {:?}", last_level, level);

        if level > last_level && is_decreasing {
            // if we are decreasing and the level is up, unsafe
            info!("unsafe, cond1");
            return false;
        } else if level < last_level && is_increasing {
            // If we are increasing and the level is down, unsafe
            info!("unsafe, cond2");
            return false;
        } else if level > last_level {
            let difference = level - last_level;
            is_increasing = true;
            if difference > 3 || difference < 1 {
                info!("unsafe, cond3");
                return false;
            }
        } else {
            let difference = last_level - level;
            is_decreasing = true;
            if difference > 3 || difference < 1 {
                info!("unsafe, cond4");
                return false;
            }
        }
        last_level = level
    }
    info!("is safe!");
    return true;
}

/// Find unsafe levels
/// ```
/// let vec1: Vec<String> = vec![
///     "7 6 4 2 1",
///     "1 2 7 8 9",
///     "9 7 6 2 1",
///     "1 3 2 4 5",
///     "8 6 4 4 1",
///     "1 3 6 7 9"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day02::puzzle_a(&vec1), 2);
/// ```
pub fn puzzle_a(string_list: &Vec<String>) -> u32 {
    let levels = parse(string_list);
    let filtered_levels: Vec<Vec<u32>> = levels
        .into_iter()
        .filter(|report| is_safe(report))
        .collect();
    return filtered_levels.len() as u32;
}

/// https://en.wikipedia.org/wiki/Power_set
fn power_set<T: Clone>(a_vector: &Vec<T>) -> Vec<Vec<T>> {
    return a_vector
        .iter()
        // Fold - initial value, |curValue, next value|
        .fold(vec![vec![]], |mut power_set, value| {
            // Clone the previous sets, add the value to every set, thats the new sets
            let new_sets = power_set.clone().into_iter().map(|mut previous_set| {
                previous_set.push(value.clone());
                return previous_set;
            });
            power_set.extend(new_sets);
            return power_set;
        });
}

fn is_safe_dampener(report: &Vec<u32>, dampener: usize) -> bool {
    if is_safe(report) {
        return true;
    }
    let min_size = report.len() - dampener;
    // generate the powersets, cull those too small, return immediately if we have found one that works
    for possible_report in power_set(report) {
        if possible_report.len() < min_size {
            continue;
        }
        if is_safe(&possible_report) {
            return true;
        }
    }
    return false;
}

/// Find at most 1 unsafe level
/// ```
/// let vec1: Vec<String> = vec![
///     "7 6 4 2 1",
///     "1 2 7 8 9",
///     "9 7 6 2 1",
///     "1 3 2 4 5",
///     "8 6 4 4 1",
///     "1 3 6 7 9"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day02::puzzle_b(&vec1), 4);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> u32 {
    let levels = parse(string_list);
    let filtered_levels: Vec<Vec<u32>> = levels
        .into_iter()
        .filter(|report| is_safe_dampener(report, 1))
        .collect();
    return filtered_levels.len() as u32;
}
