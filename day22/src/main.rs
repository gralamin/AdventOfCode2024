use day22::load_no_blanks;
use day22::puzzle_a;
use day22::puzzle_b;

fn main() {
    colog::init();
    let filename = "input";
    let lines = load_no_blanks(filename);

    let value = puzzle_a(&lines);
    println!("Answer to 1st question: {}", value);

    let value_b = puzzle_b(&lines, 0, 2000);
    println!("Answer to 2nd question: {}", value_b);
}
