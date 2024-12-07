
pub fn calculate(input: &str) -> Vec<(usize, usize)>{
    let mut sum = 0;

    let mut map: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();

    let guard_pos_y = map.iter().position(|y| y.contains(&'^')).unwrap();
    let mut guard_pos: ((usize, usize), (i32, i32)) = ((guard_pos_y, map[guard_pos_y].iter().position(|x| x == &'^').unwrap()), (-1, 0));

    let mut known_pos: Vec<(usize, usize)> = Vec::new();
    known_pos.push(guard_pos.0);
    loop{
        let is_end = move_guard(&mut guard_pos, &mut map, &mut known_pos);


        if  is_end{
            break;
        }
    }

    println!("{:?}", guard_pos);

    println!("Part1: {}", known_pos.len());
    return known_pos;
}

fn move_guard(guard_pos: &mut ((usize, usize), (i32, i32)), map: &mut Vec<Vec<char>>, known_pos: &mut Vec<(usize, usize)>) -> bool {
    let mut result = false;
    let next_pos = calc_pos(guard_pos.0.0, guard_pos.0.1, guard_pos.1.0, guard_pos.1.1);
    let next_checked_pos = check_end(next_pos.0, next_pos.1, map);
    match next_checked_pos {
        Some(pos) => {
            if check_obstacle(pos, map){
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
            else{
                if !known_pos.contains(&pos){
                    known_pos.push(pos);
                }
                guard_pos.0 = pos;
            }

            result = false;
        },
        None => {
            result = true;
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