use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();


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

    part1(print_lines, rules);
    part2(print_lines, rules);
}

fn part1(print_lines: &Vec<Vec<u32>>, rules: &Vec<Vec<u32>>){
    let mut sum = 0;

    let mut print_accepted;
    for print in print_lines{
        print_accepted = true;
        for page_index in 0..print.len() {
            if !check_rules(page_index, print, &rules).0{
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

fn check_rules<'a>(page_index: usize, print: &'a Vec<u32>, rules: &'a Vec<Vec<u32>> ) -> (bool, Vec<u32>){
    let page = print[page_index];
    let mut filtered_rules: Vec<&Vec<u32>> = rules.iter().filter(|r| r[1] == page).collect(); 

    let mut follows_rules = true;
    'rule_loop: for rule in filtered_rules {
        for future_print in page_index+1..print.len(){
            if print[future_print] as u32 == rule[0]{
                return (false, rule.to_vec())
            }
        }
    }
    return (true, Vec::new())
}

fn part2(print_lines: &Vec<Vec<u32>>, rules: &Vec<Vec<u32>>){
    let mut sum = 0;

    let mut print_check_result;
    for original_print in print_lines{
        let mut print = original_print.to_vec();
        print_check_result = check_print(&print, rules);
        if !print_check_result.0{
            loop {
                print = sort(print_check_result.1, &print, print_check_result.2.to_vec());
                print_check_result = check_print(&print, rules);
                if print_check_result.0{
                    sum += print[(0 as usize).midpoint(print.len()) ];
                    break;
                }
            }
        }
    }

    println!("Part2: {}", sum);
}


fn check_print(print: &Vec<u32>, rules: &Vec<Vec<u32>>) -> (bool, usize, Vec<u32>){
    for page_index in 0..print.len() {
        let check_rule_result = check_rules(page_index, print, &rules);
        if !check_rule_result.0{
            return (false, page_index, check_rule_result.1);
        }
    }
    return (true, 0, Vec::new());
}

fn sort(page_index: usize, mut print: &Vec<u32>, rule: Vec<u32> ) -> Vec<u32>{
    let error_page_index = print.iter().position(|p| *p == rule[0]).unwrap();

    let mut edited_print = print.to_vec();

    edited_print.remove(error_page_index);
    edited_print.insert(page_index, rule[0]);

    return edited_print;
}