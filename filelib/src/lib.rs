use std::fs;

/// Load the "input" file
pub fn load(filename: &str) -> String {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading");
    return contents;
}

/// remove blank lines
fn remove_blanks(text_input: &str) -> Vec<String> {
    return text_input
        .lines()
        .filter(|&s| !s.is_empty() && !s.trim().is_empty())
        .map(str::to_string)
        .collect();
}

/// Load without blank lines
pub fn load_no_blanks(filename: &str) -> Vec<String> {
    return remove_blanks(&load(filename));
}

fn strings_to_i32(strings: Vec<&str>) -> Vec<i32> {
    let result: Vec<i32> = strings.iter().map(|x| x.parse::<i32>().unwrap()).collect();
    return result;
}

/// Load and convert to 32-bit integers
pub fn load_as_ints(filename: &str) -> Vec<i32> {
    let strings = load_no_blanks(filename);
    return strings_to_i32(strings.iter().map(AsRef::as_ref).collect());
}

/// Input parsing, use blank lines to produce groups
///
/// This removes any empty groups.
/// ```
/// let ins = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\n22 13 17 11  0\n 8  2 23  4 24\n21  9 14 16  7\n 6 10  3 18  5\n 1 12 20 15 19\n\n 3 15  0  2 22\n 9 18 13 17  5\n19  8  7 25 23\n20 11 10 24  4\n14 21 16 12  6\n\n\n14 21 17 24  4\n10 16 15  9 19\n18  8 23 26 20\n22 11 13  6  5\n 2  0 12  3  7\n\n";
/// let outs = vec![
///    vec!["7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1".to_string()],
///    vec!["22 13 17 11  0", " 8  2 23  4 24", "21  9 14 16  7", " 6 10  3 18  5", " 1 12 20 15 19"].iter().map(|s| s.to_string()).collect(),
///    vec![" 3 15  0  2 22", " 9 18 13 17  5", "19  8  7 25 23", "20 11 10 24  4", "14 21 16 12  6"].iter().map(|s| s.to_string()).collect(),
///    vec!["14 21 17 24  4", "10 16 15  9 19", "18  8 23 26 20", "22 11 13  6  5", " 2  0 12  3  7"].iter().map(|s| s.to_string()).collect(),
/// ];
/// assert_eq!(filelib::split_lines_by_blanks(ins), outs);
/// ```
pub fn split_lines_by_blanks(lines: &str) -> Vec<Vec<String>> {
    let mut result: Vec<Vec<String>> = Vec::new();

    let mut cur_break: Vec<String> = Vec::new();
    for cur_line in lines.lines() {
        let trimmed = cur_line.trim();
        if trimmed.is_empty() {
            result.push(cur_break);
            cur_break = Vec::new();
        } else {
            cur_break.push(cur_line.to_string());
        }
    }
    result.push(cur_break);

    // remove any blank vectors
    return result.into_iter().filter(|s| !s.is_empty()).collect();
}

/// Input parsing, split lines into a flat bunch of numbers
///
/// Note this flattens everything to one line.
/// ```
/// let ins = vec![vec!["7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1".to_string()]];
/// let outs = vec![7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1];
/// assert_eq!(filelib::parse_csv_i32_lines(ins), outs);
/// ```
pub fn parse_csv_i32_lines(lines: Vec<Vec<String>>) -> Vec<i32> {
    // First, flatten a layer
    let flattened_lines: Vec<String> = lines.into_iter().flatten().collect();
    let number_lines: Vec<Vec<i32>> = flattened_lines
        .iter()
        .map(|line| {
            line.split(",")
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();
    let numbers: Vec<i32> = number_lines.into_iter().flatten().collect();
    return numbers;
}

/// Parses a line of the form "x1,y1 -> x2,y2"
///
/// Extracts the numbers so this can be used a simple line.
/// ```
/// let input = "1,2 -> 3,-4";
/// assert_eq!(filelib::parse_line_to_linecoords(input), (1, 2, 3, -4));
/// ```
pub fn parse_line_to_linecoords(line: &str) -> (i32, i32, i32, i32) {
    let vec_version: Vec<Vec<i32>> = line
        .split("->")
        .map(|pair| {
            pair.split(",")
                .map(|p| p.trim().parse::<i32>().unwrap())
                .collect()
        })
        .collect();
    let vec_flat: Vec<i32> = vec_version.into_iter().flatten().collect();
    return (vec_flat[0], vec_flat[1], vec_flat[2], vec_flat[3]);
}

/// Parses a path of the form "x1,y1 -> x2,y2 -> x3,y3" (etc)
///
/// ```
/// let input = "1,2 -> 3,-4 -> 5,6 -> 1,2";
/// assert_eq!(filelib::parse_path_to_coords(input), vec![(1,2), (3,-4), (5, 6), (1, 2)]);
/// ```
pub fn parse_path_to_coords(line: &str) -> Vec<(i32, i32)> {
    let vec_version: Vec<(i32, i32)> = line
        .split("->")
        .map(|pair| {
            let (x, y) = pair.split_once(",").unwrap();
            return (
                x.trim().parse::<i32>().unwrap(),
                y.trim().parse::<i32>().unwrap(),
            );
        })
        .collect();
    return vec_version;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_blanks() {
        let input = "\n199\n200\n208\n210\n\n200\n207\n240\n269\n260\n263\n";
        let expected = vec![
            "199", "200", "208", "210", "200", "207", "240", "269", "260", "263",
        ];
        assert_eq!(remove_blanks(input), expected);
    }

    #[test]
    fn test_strings_to_i32() {
        let input = vec![
            "199", "200", "208", "210", "200", "207", "240", "269", "260", "263",
        ];
        let expected = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(strings_to_i32(input), expected);
    }

    #[test]
    fn test_parse_line_to_coords() {
        assert_eq!(parse_line_to_linecoords("6,4 -> 2,0"), (6, 4, 2, 0));
    }
}
