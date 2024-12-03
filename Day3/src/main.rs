use std::fs;
use regex::Regex;
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str){
    let mut sum = 0;

    let re = Regex::new(r"mul\((\d{0,3}),(\d{0,3})\)").unwrap();

    sum = re.captures_iter(input).map(|p| {
        let (_, [num1, num2]) = p.extract();
        return num1.parse::<i32>().unwrap() * num2.parse::<i32>().unwrap()
    }).sum(); 
    
    println!("Part1: {}", sum);
}

fn part2(input: &str){
    let mut sum = 0;

    let re = Regex::new(r"mul\((\d{0,3}),(\d{0,3})\)|do\(\)|don't\(\)").unwrap();

    let items: Vec<(&str, &str, &str)> = re.captures_iter(input).map(|p| {
        let m: &str = match p.get(0){
            Some(v) => v.as_str(),
            None => ""
        };
        
        let grp1 = match p.get(1){
            Some(v) => v.as_str(),
            None => ""
        };

        let grp2 = match p.get(2){
            Some(v) => v.as_str(),
            None => ""
        };

        return (m,grp1, grp2);
    }).collect(); 

    let mut enabled: bool = true;
    for item  in items {
        if item.0 == "do()"{
            enabled = true;
        }
        else if item.0 == "don't()" {
            enabled = false;
        }
        else{
            if(enabled){
                sum += item.1.parse::<i32>().unwrap() * item.2.parse::<i32>().unwrap()
            }
        }
    }

    println!("Part2: {}", sum);
}