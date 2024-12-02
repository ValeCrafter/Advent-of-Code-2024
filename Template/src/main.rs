
use std::fs;
use std::error::Error;

fn main() {
    let mut input_file = &read_input_file();
    let mut output: String = "empty".to_string();



    output = input_file.to_string();



    print!("{}", output);
}

fn read_input_file() -> String{
    return fs::read_to_string("input.txt").unwrap();
}