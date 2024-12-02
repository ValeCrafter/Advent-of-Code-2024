
use std::fs;

fn main() {
    let input_file = fs::read_to_string("input.txt").unwrap();
    let mut output: String = "empty".to_string();

    // spallte splitte zu 2 vec<i32>

    let mut right:Vec<i32> = Vec::new();
    let mut left:Vec<i32> = Vec::new();
    for line in input_file.lines(){
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

    output = sum.to_string();

    println!("{}", output);
}