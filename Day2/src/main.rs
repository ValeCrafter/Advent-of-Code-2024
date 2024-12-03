use std::{clone, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str){
    let mut sum = 0;

    for line in input.lines(){
        let mut is_line_safe = true;
        let mut increasing: Option<bool> = None;
        let nums: Vec<&str> = line.split(" ").collect();
        for i in 1..nums.len(){
            let current_number: i32 = nums[i].parse().unwrap();
            let previous_number: i32 = nums[i-1].parse().unwrap();

            match increasing{ //this optional is also useless, because you could allways just check the first and second number to see if it is increasing or decreasing
                None =>{
                    if previous_number < current_number{ //increasing
                        increasing = Some(true);
                        if current_number - previous_number > 3 {
                            is_line_safe = false;
                            break;
                        }
                    }
                    else if previous_number > current_number{ //decreasing
                        increasing = Some(false);
                        if previous_number - current_number > 3 {
                            is_line_safe = false;
                            break;
                        }
                    }
                    else{
                        is_line_safe = false;
                        break;
                    }
                },
                Some(incr) => {
                    if incr && current_number - previous_number > 3
                        || incr && previous_number >= current_number 
                        || !incr && previous_number - current_number > 3
                        || !incr && previous_number <= current_number { // this if is almost disgusting
                        is_line_safe = false;
                        break;
                    }
                }
            };
        }
        if is_line_safe{
            sum += 1;
        }
    }
    
    println!("Part1: {}", sum);
}

fn part2(input: &str){
    let mut sum = 0;

    for line in input.lines(){
        let ref original_nums: Vec<&str> = line.split(" ").collect();
        
        let is_safe = run_check(original_nums.to_vec());
        if is_safe {
            sum += 1;
        }
        else {
            for i in 0..original_nums.len() {
                let mut nums = original_nums.clone();

                nums.remove(i);
                if run_check(nums){
                    sum +=1;
                    break;
                }
            }
        }
    }
    println!("Part2: {}", sum);
}

fn run_check(nums: Vec<&str>) -> bool{
    let mut is_line_safe = true;
    let mut increasing: Option<bool> = None;
    for i in 1..nums.len(){
        let current_number: i32 = nums[i].parse().unwrap();
        let previous_number: i32 = nums[i-1].parse().unwrap();

        match increasing{ //this optional is also useless, because you could allways just check the first and second number to see if it is increasing or decreasing
            None =>{
                if previous_number < current_number{ //increasing
                    increasing = Some(true);
                    if current_number - previous_number > 3 {
                        is_line_safe = false;
                        break;
                    }
                }
                else if previous_number > current_number{ //decreasing
                    increasing = Some(false);
                    if previous_number - current_number > 3 {
                        is_line_safe = false;
                        break;
                    }
                }
                else{
                    is_line_safe = false;
                    break;
                }
            },
            Some(incr) => {
                if incr && current_number - previous_number > 3
                    || incr && previous_number >= current_number 
                    || !incr && previous_number - current_number > 3
                    || !incr && previous_number <= current_number { // this if is almost disgusting
                    is_line_safe = false;
                    break;
                }
            }
        };
    }
    return is_line_safe
} 