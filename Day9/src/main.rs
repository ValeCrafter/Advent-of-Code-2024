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

    
    //fill vec
    let mut drive= Vec::new();
    for c_index in 0..input.len(){
        let c: char = input.chars().collect::<Vec<char>>()[c_index];
        if c_index % 2 == 0{
            drive.push((Some(c_index as u64 / 2), c.to_digit(10).unwrap()));
        } 
        else{
            drive.push((None, c.to_digit(10).unwrap()));
        }
    }

    //sort
    for i in 0..drive.len(){
        let pos = drive.len() - 1usize - i;
        let pos_item = drive[pos];

        if pos_item.0.is_some(){
            let empty_pos = drive.iter().position(|p| p.1 >= pos_item.1 && p.0.is_none());
            match empty_pos{
                Some(v) => {
                    if v < pos{
                        drive[pos].0 = None;
                        if drive[v].1 == pos_item.1{
                            drive.remove(v);
                        }
                        else{
                            drive[v].1 -= pos_item.1;
                            if drive[v].1 == 0{
                                drive.remove(v);
                            }
                        }
                        drive.insert(v,pos_item);
                    }
                }
                None => ()
            }
        }
    }

    //calc
    let mut index = 0;
    for item in drive{
        if item.0.is_some(){
            for i in 0..item.1{
                sum += item.0.unwrap() * (index + i) as u64;
            }
        }
        index += item.1;
    }

    println!("Part2: {}", sum);
}