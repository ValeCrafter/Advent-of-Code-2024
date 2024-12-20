use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    part1(&input);
    part2(&input);
}

/*
0 = add
1 = multiply
*/

fn part1(input: &str){
    let mut sum = 0;

    let calcs: Vec<(u64, Vec<u64>)> = input.lines().map(|l| {
        let mut nums = l.split(": ");
        return (nums.next().unwrap().parse().unwrap(), nums.next().unwrap().split(" ").map(|num| num.parse().unwrap()).collect())
        }
    ).collect();

    for calc in calcs{
        let mut operations = vec![0; calc.1.len() -1];
        let operations_len = operations.len();
        let mut is_finished = false;
        loop{
            let mut last_result = calc.1[0];
            if count_opperator(&mut operations, 0){
                is_finished = true;
            }

            for o in 0..operations_len {
                let operation = operations[o];
                let num = calc.1[o+1];

                if operation == 0{
                    last_result = last_result + num
                }
                else if operation == 1{
                    last_result = last_result * num
                }

            }
            if last_result == calc.0{
                sum += last_result;
                break;
            }
            if is_finished {
                break;
            }
        }
    }

    println!("Part1: {}", sum);
}

fn count_opperator(operators: &mut Vec<u128>, index: usize) -> bool{
    match operators.get(index){
        Some(v) => {
            if *v < 1  {
                operators[index] += 1;
            }
            else{
                operators[index] -= 1;
                if count_opperator(operators, index + 1){
                    return true;
                }
            }
            return false;
        }
        None =>  true
    }
}

fn part2(input: &str){
    let mut sum = 0;

    let calcs: Vec<(u128, Vec<u128>)> = input.lines().map(|l| {
        let mut nums = l.split(": ");
        return (nums.next().unwrap().parse().unwrap(), nums.next().unwrap().split(" ").map(|num| num.parse().unwrap()).collect())
        }
    ).collect();

    for calc in calcs{
        let mut operations = vec![0; calc.1.len() -1];
        let operations_len = operations.len();
        let mut is_finished = false;
        loop{
            let mut last_result = calc.1[0];
            if count_opperator2(&mut operations, 0){
                is_finished = true;
            }
            //println!("{:?}", operations);
            for o in 0..operations_len {
                let operation = operations[o];
                let num = calc.1[o+1];

                if operation == 0{
                    last_result = last_result + num
                }
                else if operation == 1{
                    match last_result.checked_mul(num){
                        Some(v) => {
                            last_result =  v;
                        }
                        None => {
                            break;
                        }
                    }
                }
                else if operation == 2{
                    match last_result.checked_mul(10u128.pow(num.ilog10() + 1)){
                        Some(v) => {
                            last_result =  v + num;
                        },
                        None => {
                            //print!("asdf");
                            break;
                        }
                    }
                }
            }
            if last_result == calc.0{
                sum += last_result;
                break;
            }
            if is_finished {
                break;
            }
        }
    }

    println!("Part2: {}", sum);
}


fn count_opperator2(operators: &mut Vec<u128>, index: usize) -> bool{
    match operators.get(index){
        Some(v) => {
            if *v < 2  {
                operators[index] += 1;
            }
            else{
                operators[index] = 0;
                if count_opperator2(operators, index + 1){
                    return true;
                }
            }
            return false;
        }
        None =>  true
    }
}