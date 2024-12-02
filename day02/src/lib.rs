extern crate filelib;

pub use filelib::load_no_blanks;

const SHOULD_DEBUG: bool = false;

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

    if SHOULD_DEBUG {
        println!("Current report {:?}", report);
    }
    for level in report {
        if !handle_first {
            handle_first = true;
            continue;
        }
        if SHOULD_DEBUG {
            println!("handling last {:?} and now {:?}", last_level, level);
        }

        if level > last_level && is_decreasing {
            // if we are decreasing and the level is up, unsafe
            if SHOULD_DEBUG {
                println!("unsafe, cond1");
            }
            return false;
        } else if level < last_level && is_increasing {
            // If we are increasing and the level is down, unsafe
            if SHOULD_DEBUG {
                println!("unsafe, cond2");
            }
            return false;
        } else if level > last_level {
            let difference = level - last_level;
            is_increasing = true;
            if difference > 3 || difference < 1 {
                if SHOULD_DEBUG {
                    println!("unsafe, cond3");
                }
                return false;
            }
        } else {
            let difference = last_level - level;
            is_decreasing = true;
            if difference > 3 || difference < 1 {
                if SHOULD_DEBUG {
                    println!("unsafe, cond4");
                }
                return false;
            }
        }
        last_level = level
    }
    if SHOULD_DEBUG {
        println!("is safe!");
    }
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

/// Foo
/// ```
/// let vec1: Vec<String> = vec![
///     "foo"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day02::puzzle_b(&vec1), 0);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> u32 {
    return 0;
}

/// Delete this after starting on puzzle_a.
/// ```
/// let vec1: Vec<u32> = vec![];
/// let vec2: Vec<u32> = vec![1];
/// assert_eq!(day02::coverage_workaround(&vec1), 1);
/// assert_eq!(day02::coverage_workaround(&vec2), 2);
/// ```
pub fn coverage_workaround(a: &Vec<u32>) -> u32 {
    if a.len() == 0 {
        return 1;
    } else {
        return 2;
    }
}
