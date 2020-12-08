use crate::files::{print_all, get_lines};
use std::str::FromStr;
use std::collections::HashSet;
use regex::{Regex, Captures};

lazy_static! {
    static ref INTRUCT_RGX: Regex = Regex::new(r"^([a-z]{3}) ([+-][0-9]*)").unwrap();
}

fn solution(lines: &Vec<String>) -> i32 {
    let mut accumulator: i32 = 0;
    let mut line_idx = 0;
    let mut visited: HashSet<i32> = HashSet::new();

    loop {
        if visited.contains(&line_idx) {
            break;
        }

        let line = &lines[line_idx as usize];
        let instruct_caps = &INTRUCT_RGX.captures_iter(line)
            .collect::<Vec<regex::Captures<'_>>>()[0];

        visited.insert(line_idx);

        let arg = &instruct_caps[2].parse::<i32>().unwrap();
        match &instruct_caps[1] {
            "jmp" => { line_idx += arg; },
            "acc" => {
                accumulator += arg;
                line_idx += 1;
            },
            &_ => { line_idx += 1; }                            // covers 'nop'
        }
    }

    return accumulator;
}

fn solution2(lines: &Vec<String>) -> u64 {
   0
}


pub(crate) fn run() {
    let lines = get_lines("./input/8in.txt").unwrap();

    let answer1 = solution(&lines);
    println!("\nday 8, - answer 1: {}", answer1);

    //let answer2 = solution2(&lines);
    //println!("\nday 6, - answer 2: {}", answer2);
}
