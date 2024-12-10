use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str){
    let mut sum = 0;

    let lines: Vec<Vec<char>> = input.lines()
                                .map(|s|   
                                    s.chars()
                                    .collect()
                                            )
                                .collect();
    let mut satelites: Vec<(char, usize, usize)> = Vec::new();
    for y in 0..lines.len(){
        for x in 0..lines[y].len(){
            if lines[y][x] != '.'{
                satelites.push((lines[y][x], x , y));
            }
        }
    }


    let mut antinodes: Vec<(usize, usize)> = Vec::new();
    
    for sat in &satelites{
        let same_satelites: Vec<&(char, usize, usize)> = satelites.iter().filter(|s| s.0 == sat.0 && s != &sat).collect();

        for same_sat in same_satelites{
            let calced_antinode = calc_positions((sat.1, sat.2), (same_sat.1, same_sat.2), lines[0].len()-1, lines.len()-1);

            if calced_antinode.is_some() && !antinodes.contains(&calced_antinode.unwrap()) {
                antinodes.push(calced_antinode.unwrap());
            }
        }
    }
    
    println!("Part1: {}", antinodes.len());
}

fn calc_positions(source: (usize, usize), target: (usize, usize), max_x: usize, max_y: usize) -> Option<(usize, usize)>{
    let result:  (i32, i32) = (target.0 as i32 - source.0 as i32, target.1 as i32 - source.1 as i32);
    let x1 = (target.0 as i32).checked_add(result.0);
    let y1= (target.1 as i32).checked_add(result.1);

    if x1 == None || y1 == None || x1.unwrap() < 0 || y1.unwrap() < 0 || x1.unwrap() > max_x as i32|| y1.unwrap() > max_y as i32{
        return  None;
    }
    else{
        return Some((x1.unwrap() as usize, y1.unwrap() as usize));
    }
} 

fn part2(input: &str){
    let mut sum = 0;

    println!("Part2: {}", sum);
}