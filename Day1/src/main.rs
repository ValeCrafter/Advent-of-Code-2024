
use std::fs;

fn main() {
    let input_file = fs::read_to_string("input.txt").unwrap();
    part1(&input_file);
    part2(&input_file);
}

fn part1(input: &str){
    // spallte splitte zu 2 vec<i32>

    let mut right:Vec<i32> = Vec::new();
    let mut left:Vec<i32> = Vec::new();
    for line in input.lines(){
        let mut i = line.split("   ");
        left.push(i.next().unwrap().parse().unwrap());
        right.push(i.next().unwrap().parse().unwrap());
    }

    //sort
    left.sort_unstable();
    right.sort_unstable();

    // loop add things

    let mut sum = 0;
    for i in 0..left.len(){
        sum += (left[i] - right[i]).abs()
    }

    println!("Part1: {}", sum);
}

fn part2(input: &str){
    // spallte splitte zu 2 vec<i32>

    let mut right:Vec<i32> = Vec::new();
    let mut left:Vec<i32> = Vec::new();
    for line in input.lines(){
        let mut i = line.split("   ");
        left.push(i.next().unwrap().parse().unwrap());
        right.push(i.next().unwrap().parse().unwrap());
    }

    let mut sum = 0;
    for l in left{
        sum += right.iter().filter(|&&r| r == l).count() as i32 * l
    }

    println!("Part2: {}", sum);
}