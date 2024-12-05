use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str){
    let mut sum = 0;

    let mut lines: Vec<Vec<char>> = input.lines()
                                .map(|s|   
                                    s.chars()
                                    .collect()
                                            )
                                .collect();
    for y in 0..lines.len(){
        //println!("{:?}", lines[y]);
        for x in 0..lines[0].len(){
            let x_char = lines[y][x];
            if x_char == 'X'{
                sum += check_position(&lines, y as i32, x as i32)
            }
        }
    }
    println!("Part1: {}", sum);
}

fn check_position(lines: &Vec<Vec<char>>, y:i32,x:i32) -> u32{
    let directions = check_around(&lines, 'M',y,x);
    let mut sum: u32 = 0;
    if directions.len() > 0{
        for direction in directions {
            let mut is_xmas = false;
            if check_direction(&lines,'A',y,x, direction.0*2, direction.1*2){
                if check_direction(&lines, 'S', y,x,direction.0*3, direction.1*3){
                    sum += 1;
                }
            }
        }
    }
    return sum;
}

fn check_around(lines: &Vec<Vec<char>>, seach_char: char,y: i32, x: i32) -> Vec<(i32, i32)>{ //y,x
    let offsets  = vec![(-1,-1), (-1,0), (-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)];
    let mut result: Vec<(i32, i32)> = Vec::new(); 
    for offset in offsets{
        if check_direction(&lines, seach_char, y,x,offset.0, offset.1){
           result.push((offset.0, offset.1));
        }
    }
    return result;
}

fn check_direction(lines: &Vec<Vec<char>>,seach_char: char,y: i32, x: i32,y_offset: i32, x_offset: i32) -> bool{
    let y_check = y + y_offset;
    let x_check = x + x_offset;

    if y_check >=0 && x_check >= 0 && y_check < lines.len() as i32 && x_check < lines[0].len() as i32{
        return lines[y_check as usize][x_check as usize] == seach_char;
    }
    return false
}

fn part2(input: &str){
    let mut sum = 0;

    let mut lines: Vec<Vec<char>> = input.lines()
                                .map(|s|   
                                    s.chars()
                                    .collect()
                                            )
                                .collect();
    for y in 0..lines.len(){
        //println!("{:?}", lines[y]);
        for x in 0..lines[0].len(){
            let x_char = lines[y][x];
            if x_char == 'A'{
                let m_positions: Vec<(i32, i32)> = check_around(&lines, 'M', y as i32, x as i32)
                    .iter()
                    .filter(|p| p.0 != 0 && p.1 != 0)
                    .copied()
                    .collect();
                let s_positions: Vec<(i32, i32)>  = check_around(&lines, 'S', y as i32, x as i32)
                    .iter()
                    .filter(|p| p.0 != 0 && p.1 != 0)
                    .copied()
                    .collect();

                if m_positions.len() == 2 && s_positions.len() == 2 {

                    let expectec_s_positions: Vec<(i32, i32)> = m_positions.iter().map(|m| (m.0*-1,m.1*-1)).collect();

                    if expectec_s_positions[0].0 == s_positions[1].0 
                        && expectec_s_positions[0].1 == s_positions[1].1
                        && expectec_s_positions[1].0 == s_positions[0].0
                        && expectec_s_positions[1].1 == s_positions[0].1{
                        sum += 1;
                    }
                }
            }
        }
    }

    println!("Part2: {}", sum);
}