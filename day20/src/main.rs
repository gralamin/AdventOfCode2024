use day20::load_no_blanks;
use day20::puzzle_a;
use day20::puzzle_b;

fn main() {
    colog::init();
    let filename = "input";
    let lines = load_no_blanks(filename);

    let value = puzzle_a(&lines, 100);
    println!("Answer to 1st question: {}", value);

    let value_b = puzzle_b(&lines, 100);
    println!("Answer to 2nd question: {}", value_b);
}
