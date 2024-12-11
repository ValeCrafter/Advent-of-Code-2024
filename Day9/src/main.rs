use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str){
    let mut sum: u128 = 0;

    //fill vec
    let mut drive = Vec::new();
    let mut displays_empty = false;
    for c_index in 0..input.len(){
        let c: char = input.chars().collect::<Vec<char>>()[c_index];
        let mut c_input: Vec<Option<u32>>;
        if c_index % 2 == 0{
            c_input = vec![Some(c_index as u32 / 2); c.to_digit(10).unwrap() as usize];
        } 
        else{
            c_input = vec![None; c.to_digit(10).unwrap() as usize];
        }
        drive.append(&mut c_input);
    }

    //zip
    for pos in 0..drive.len(){
        let empty_pos = drive.iter().position(|v| v.is_none());

        match empty_pos{
            Some(v) => {
                drive[v] = drive[drive.len() -1];
                drive.remove(drive.len() -1);
            }
            None => break
        }
    }

    //calc
    for pos in 0..drive.len(){
        sum += drive[pos].unwrap() as u128 * pos as u128;
    }
    
    println!("Part1: {}", sum);
}

fn part2(input: &str){
    let mut sum = 0;

    println!("Part2: {}", sum);
}