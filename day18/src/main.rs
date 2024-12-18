use day18::load_no_blanks;
use day18::puzzle_a;
use day18::puzzle_b;

fn main() {
    colog::init();
    let filename = "input";
    let lines = load_no_blanks(filename);

    let value = puzzle_a(&lines, 71, 71, 1024);
    println!("Answer to 1st question: {}", value);

    let (x, y) = puzzle_b(&lines, 71, 71, 1024);
    println!("Answer to 2nd question: {},{}", x, y);
}
