use day24::puzzle_a;
use day24::puzzle_b;
use day24::{load, split_lines_by_blanks};

fn main() {
    colog::init();
    let filename = "input";
    let lines = load(filename);
    let groups = split_lines_by_blanks(&lines);

    let value = puzzle_a(&groups);
    println!("Answer to 1st question: {}", value);

    let value_b = puzzle_b(&groups, 4);
    println!("Answer to 2nd question: {}", value_b);
}
