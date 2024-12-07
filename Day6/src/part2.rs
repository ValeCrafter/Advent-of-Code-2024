
pub fn calculate(input: &str, passed_pos: Vec<(usize, usize)>) {
    let mut sum = 0;

    let mut map: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();

    let guard_pos_y = map.iter().position(|y| y.contains(&'^')).unwrap();
    let mut guard_pos: ((usize, usize), (i32, i32)) = ((guard_pos_y, map[guard_pos_y].iter().position(|x| x == &'^').unwrap()), (-1, 0));

    for pos in passed_pos {
        let mut hit_pos: Vec<((usize, usize),(usize, usize))> = Vec::new();
        let mut loop_guard_pos = guard_pos.clone();
        loop{
            let is_end = move_guard(&mut loop_guard_pos, &mut map, pos,&mut hit_pos);
    
    
            if  is_end.is_some() && is_end.unwrap(){
                sum += 1;
                break;
            }
            else if is_end.is_some() && !is_end.unwrap(){
                break;
            }
        }
    }
    println!("Part2: {}", sum);
}

fn move_guard(guard_pos: &mut ((usize, usize), (i32, i32)), map: &mut Vec<Vec<char>>, obstacle_pos: (usize, usize) , hit_pos: &mut Vec<((usize, usize),(usize, usize))>) -> Option<bool> {
    let mut result = None;
    let next_pos = calc_pos(guard_pos.0.0, guard_pos.0.1, guard_pos.1.0, guard_pos.1.1);
    let next_checked_pos = check_end(next_pos.0, next_pos.1, map);
    match next_checked_pos {
        Some(pos) => {
            if pos == (6,3){
                
            }
            if check_obstacle(pos, map) || pos == obstacle_pos{
                if hit_pos.contains(&(pos,guard_pos.0)){
                    result = Some(true);
                }
                else{
                    hit_pos.push((pos,guard_pos.0));
                    match guard_pos.1{
                        (-1, 0) => {
                            guard_pos.1.0 = 0;
                            guard_pos.1.1 = 1;
                        },//up
                        (0, 1) => {
                            guard_pos.1.0 = 1;
                            guard_pos.1.1 = 0;
                        },//right
                        (1, 0) => {
                            guard_pos.1.0 = 0;
                            guard_pos.1.1 = -1;
                        },//down
                        (0, -1) => {
                            guard_pos.1.0 = -1;
                            guard_pos.1.1 = 0;
                        },//left
                        (_,_) => panic!("This direction does not exist")
                    }
                }                
            }
            else{
                guard_pos.0 = pos;
            }
        },
        None => {
            result = Some(false);
        }
    }

    return result;
    
}

fn calc_pos(y: usize, x: usize, y_delta: i32, x_delta: i32) -> (i32, i32){
    return (y as i32 + y_delta, x as i32 + x_delta)
}

fn check_obstacle(check_pos: (usize, usize), map: &mut Vec<Vec<char>>) -> bool{
    return map[check_pos.0][check_pos.1] == '#'
}

//check_loop

fn check_end(y: i32, x: i32, map: &mut Vec<Vec<char>>) -> Option<(usize, usize)>{
    if y>=0
    && y < map.len() as i32
    && x>=0
    && x < map[0].len() as i32{
        return Some((y as usize, x as usize));
    }
    else{
        return None;
    }
}