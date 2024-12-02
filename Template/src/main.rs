
use std::fs;

fn main() {
    let input_file = fs::read_to_string("input.txt").unwrap();
    let mut output: String = "empty".to_string();

    print!("{}", output);
}