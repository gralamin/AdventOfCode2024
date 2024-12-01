extern crate filelib;

pub use filelib::load_no_blanks;

/// Get the sum of all first and last numbers in each line. If a single number appears in a line, count it for both.
/// ```
/// let vec1: Vec<String> = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day01::puzzle_a(&vec1), 142);
/// ```
pub fn puzzle_a(string_list: &Vec<String>) -> u32 {
    let tens: Vec<u32> = string_list
        .iter()
        .map(|x| {
            x.chars()
                .nth(x.find(char::is_numeric).unwrap())
                .unwrap()
                .to_digit(10)
                .unwrap()
        })
        .collect();
    let ones: Vec<u32> = string_list
        .iter()
        .map(|x| {
            x.chars()
                .nth(x.rfind(char::is_numeric).unwrap())
                .unwrap()
                .to_digit(10)
                .unwrap()
        })
        .collect();
    return tens.iter().zip(ones.iter()).map(|(x, y)| x * 10 + y).sum();
}

fn convert_to_int(value: &str) -> Option<u32> {
    return match value {
        "0" => Some(0),
        "1" => Some(1),
        "2" => Some(2),
        "3" => Some(3),
        "4" => Some(4),
        "5" => Some(5),
        "6" => Some(6),
        "7" => Some(7),
        "8" => Some(8),
        "9" => Some(9),
        "zero" => Some(0),
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    };
}

/// Get the sum of all first and last numbers in each line, including number words. If a single number appears in a line, count it for both.
/// ```
/// let vec1 = vec!["two1nine", "eightwothree", "abcone2threexyz", "xtwone3four", "4nineeightseven2", "zoneight234", "7pqrstsixteen"].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day01::puzzle_b(&vec1), 281);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> u32 {
    let matches = extract_numbers_and_number_words(&string_list);
    let ones: Vec<u32> = matches
        .iter()
        .map(|x| convert_to_int(x.iter().last().unwrap()).unwrap())
        .collect();
    let tens: Vec<u32> = matches
        .iter()
        .map(|x| convert_to_int(x.iter().nth(0).unwrap()).unwrap())
        .collect();
    return tens.iter().zip(ones.iter()).map(|(x, y)| x * 10 + y).sum();
}

fn extract_numbers_and_number_words(string_list: &Vec<String>) -> Vec<Vec<String>> {
    // Wouldn't it be great if I could just use a regex?
    // Well I can't, due to requring a lookahead (this solves the case of "twone" matching two but not one)
    // which for some reason isn't implemented by the regex crate
    // So I guess we are just going to have to manually do this...
    let mut result: Vec<Vec<String>> = Vec::new();
    // All the digit words we need to find
    let digits = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for line in string_list {
        let mut found = vec![];
        // We need chars ahead and also later when making the remaining substring.
        let chars: Vec<_> = line.chars().collect();
        for (index, current_char) in chars.iter().enumerate() {
            // if the current character is a digit, just return that.
            if current_char.is_numeric() {
                found.push(current_char.to_string());
                continue;
            }

            // Look at remaining characters, check if the current front is a number word
            let s = String::from_iter(&chars[index..chars.len()]);
            for d in digits.iter() {
                if s.starts_with(d) {
                    found.push(d.to_string());
                }
            }
        }
        result.push(found);
    }
    return result;
}
