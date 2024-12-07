use std::{default, fs, usize};

mod part1;
mod part2;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let passed_pos = part1::calculate(&input);
    part2::calculate(&input, passed_pos);
}

