use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str){
    let mut sum = 0;
    
    println!("Part1: {}", sum);
}

fn part2(input: &str){
    let mut sum = 0;

    println!("Part2: {}", sum);
}