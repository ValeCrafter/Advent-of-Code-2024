use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str){
    let mut sum = 0;

    let generic_input = input.replace("\r\n", "\n");
    let mut input_iter = generic_input.as_str().split("\n\n");
    let ref rules: Vec<Vec<u32>> = 
        input_iter.next()
            .unwrap()
            .lines()
            .map(|l| 
                l.split("|")
                .map(|n| 
                    {n.parse::<u32>()
                        .unwrap()
                    }).collect())
            .collect();
    
    let ref print_lines: Vec<Vec<u32>> = 
    input_iter.next()
        .unwrap()
        .lines()
        .map(|l| 
            l.split(",")
            .map(|n| 
                {n.parse::<u32>()
                    .unwrap()
                }).collect())
        .collect();

    let mut print_accepted;
    for print in print_lines{
        print_accepted = true;
        for page_index in 0..print.len() {
            if !check_rules(page_index, print, &rules){
                print_accepted = false;
                break 
            }
        }
        if print_accepted{
            sum += print[(0 as usize).midpoint(print.len()) ]
        }
    }
    
    println!("Part1: {}", sum);
}

fn check_rules(page_index: usize, print: &Vec<u32>, rules: &Vec<Vec<u32>> ) -> bool{
    let page = print[page_index];
    let mut filtered_rules: Vec<&Vec<u32>> = rules.iter().filter(|r| r[1] == page).collect(); 

    let mut follows_rules = true;
    'rule_loop: for rule in filtered_rules {
        for future_print in page_index+1..print.len(){
            if print[future_print] as u32 == rule[0]{
                follows_rules = false;
                break 'rule_loop;
            }
        }
    }
    return follows_rules
}

fn part2(input: &str){
    let mut sum = 0;

    println!("Part2: {}", sum);
}
