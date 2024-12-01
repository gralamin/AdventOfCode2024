extern crate filelib;

pub use filelib::load_no_blanks;

/// Foo
/// ```
/// let vec1: Vec<String> = vec![
///     "foo"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(template::puzzle_a(&vec1), 0);
/// ```
pub fn puzzle_a(string_list: &Vec<String>) -> u32 {
    return 0;
}

/// Foo
/// ```
/// let vec1: Vec<String> = vec![
///     "foo"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(template::puzzle_b(&vec1), 0);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> u32 {
    return 0;
}

/// Delete this after starting on puzzle_a.
/// ```
/// let vec1: Vec<u32> = vec![];
/// let vec2: Vec<u32> = vec![1];
/// assert_eq!(template::coverage_workaround(&vec1), 1);
/// assert_eq!(template::coverage_workaround(&vec2), 2);
/// ```
pub fn coverage_workaround(a: &Vec<u32>) -> u32 {
    if a.len() == 0 {
        return 1;
    } else {
        return 2;
    }
}
