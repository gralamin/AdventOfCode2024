use day14::load_no_blanks;
use day14::puzzle_a;
use day14::puzzle_b;

fn main() {
    colog::init();
    let filename = "input";
    let lines = load_no_blanks(filename);

    let height = 103;
    let width = 101;
    let value = puzzle_a(&lines, height, width);
    println!("Answer to 1st question: {}", value);

    let value_b = puzzle_b(&lines, height, width);
    println!("Answer to 2nd question: {}", value_b);
}
