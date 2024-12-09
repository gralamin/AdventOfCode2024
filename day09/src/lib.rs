extern crate filelib;

pub use filelib::load_no_blanks;
use log::info;

type Number = u64;

fn parse_map(string_list: &Vec<String>) -> Vec<Option<Number>> {
    // should be one entry
    let string = string_list.first().unwrap();
    let mut result = vec![];
    for i in 0..string.len() {
        if i % 2 == 1 {
            let empty_count: usize = string.chars().nth(i).unwrap().to_digit(10).unwrap() as usize;
            for _ in 0..empty_count {
                result.push(None);
            }
        } else {
            let count: usize = string.chars().nth(i).unwrap().to_digit(10).unwrap() as usize;
            /* id map: i = 0 -> 0
                      i = 2 -> 1
                      i = 4 -> 2
                      i = 6 -> 3
            */
            let id = i as Number / 2;
            for _ in 0..count {
                result.push(Some(id));
            }
        }
    }
    return result;
}

fn compact_map(values: Vec<Option<Number>>) -> Vec<Option<Number>> {
    let mut result = values.clone();
    //On first empty, take the right most non empty number and swap them
    for i in 0..values.len() {
        let v = result[i];
        if v.is_some() {
            continue;
        }
        let mut j = values.len() - 1;
        while j > i {
            let z = result[j];
            if z.is_none() {
                j -= 1;
                continue;
            }
            result[i] = z;
            result[j] = None;
            break;
        }
    }
    return result;
}

fn checksum(values: Vec<Option<Number>>) -> Number {
    let mut num = 0;
    for i in 0..values.len() {
        let v = values[i];
        num += match v {
            Some(x) => x * i as Number,
            None => 0,
        };
    }
    return num;
}

/// Compact and Find checksum of diskmap
/// ```
/// let vec1: Vec<String> = vec![
///     "2333133121414131402"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day09::puzzle_a(&vec1), 1928);
/// ```
pub fn puzzle_a(string_list: &Vec<String>) -> Number {
    let expanded_map = parse_map(string_list);
    info!("parsed map: {:?}", expanded_map);
    let small_map: Vec<Option<Number>> = compact_map(expanded_map);
    info!("Compacted map: {:?}", small_map);
    return checksum(small_map);
}

// Parse into intervals instead
// Interval = start, end, number
fn parse_intervals(string_list: &Vec<String>) -> Vec<(usize, usize, Option<Number>)> {
    let mut result = vec![];
    let string = string_list.first().unwrap();
    let mut cur_start = 0;
    let mut cur_end;
    let mut cur_number: Option<Number>;
    for i in 0..string.len() {
        if i % 2 == 1 {
            let empty_count: usize = string.chars().nth(i).unwrap().to_digit(10).unwrap() as usize;
            if empty_count == 0 {
                continue;
            }
            cur_end = cur_start + empty_count - 1;
            cur_number = None;
            result.push((cur_start, cur_end, cur_number));
            cur_start = cur_end + 1;
        } else {
            let count: usize = string.chars().nth(i).unwrap().to_digit(10).unwrap() as usize;
            /* id map: i = 0 -> 0
                      i = 2 -> 1
                      i = 4 -> 2
                      i = 6 -> 3
            */
            let id = i as Number / 2;
            cur_end = cur_start + count - 1;
            cur_number = Some(id);
            result.push((cur_start, cur_end, cur_number));
            cur_start = cur_end + 1;
        }
    }
    return result;
}

fn interval_get_length(start: usize, end: usize) -> usize {
    return end - start + 1;
}

fn merge_empties(
    intervals: Vec<(usize, usize, Option<Number>)>,
) -> Vec<(usize, usize, Option<Number>)> {
    let mut results = intervals.clone();
    // Sort them
    results.sort_by(|a, b| a.cmp(b));
    let mut index = 0;
    while index < results.len() - 1 {
        if results[index].1 + 1 == results[index + 1].0 {
            let replaced = (results[index].0, results[index].1, None);
            results.remove(index + 1);
            results[index] = replaced;
        } else {
            index += 1;
        }
    }
    return results;
}

fn compact_map_by_files(
    intervals: Vec<(usize, usize, Option<Number>)>,
) -> Vec<(usize, usize, Option<Number>)> {
    let mut cur_values = vec![];
    let mut cur_empties = vec![];

    for i in intervals {
        if i.2.is_none() {
            cur_empties.push(i);
        } else {
            cur_values.push(i);
        }
    }

    // We have seperated the values, perform swaps
    // If an empty can store a file, pop the interval, fragment the empty, and put it back in order
    // If not delete it.
    let mut disk_end_pointer = cur_values.len() - 1;
    while disk_end_pointer > 0 {
        let (start, end, value) = cur_values[disk_end_pointer];
        let file_size = interval_get_length(start, end);
        let mut empty_size = 0;
        let mut empty_to_modify = None;
        for (index, &(e_start, e_end, _)) in cur_empties.iter().enumerate() {
            empty_size = interval_get_length(e_start, e_end);
            if empty_size >= file_size && e_start < end {
                empty_to_modify = Some(index);
                break;
            }
        }
        if empty_to_modify.is_some() {
            let empty = cur_empties[empty_to_modify.unwrap()];
            if empty_size == file_size {
                // Just delete the interval
                cur_empties.remove(empty_to_modify.unwrap());
                let new_interval = (empty.0, empty.1, value);
                info!("Moving interval to {:?}", new_interval);
                cur_values[disk_end_pointer] = new_interval;
            } else {
                let new_empty = (empty.0 + file_size, empty.1, None);
                info!("modifying empty to interval to {:?}", new_empty);
                cur_empties[empty_to_modify.unwrap()] = new_empty;
                // The rest of the empty goes to the end, never used, so its effectively deleted.
                let new_interval = (empty.0, empty.0 + file_size - 1, value);
                info!("Moving interval to {:?}", new_interval);
                cur_values[disk_end_pointer] = new_interval;
            }
            cur_empties = merge_empties(cur_empties);
            info!("empties currently {:?}", cur_empties);
        }

        disk_end_pointer -= 1;
    }

    // We don't actually need to reconstruct
    return cur_values;
}

fn checksum_interval(values: Vec<(usize, usize, Option<Number>)>) -> Number {
    let mut num = 0;
    for (start, end, value) in values {
        let next_value;
        match value {
            Some(x) => {
                let mut v = 0;
                for i in start..=end {
                    v += i as Number * x;
                }
                next_value = v;
            }
            None => next_value = 0,
        };
        num += next_value;
    }
    return num;
}

/// Move whole files instead
/// ```
/// let vec1: Vec<String> = vec![
///     "2333133121414131402"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day09::puzzle_b(&vec1), 2858);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> Number {
    let expanded_map = parse_intervals(string_list);
    println!("parsed map: {:?}", expanded_map);
    let small_map = compact_map_by_files(expanded_map);
    println!("Compacted map: {:?}", small_map);
    return checksum_interval(small_map);
}
