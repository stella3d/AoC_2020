use crate::files::{print_all, get_lines};
use std::str::FromStr;
use std::collections::HashSet;

fn solution(lines: &Vec<String>) -> u64 {
    let mut set: HashSet<char> = HashSet::new();
    let mut count: u64 = 0;
    for line in lines {
        if !line.is_empty() {
            for char in line.chars() {
                set.insert(char);
            }
        }
        else {
            count += set.len() as u64;
            set.clear();
        }
    }

    count += set.len() as u64;
    count
}

fn solution2(lines: &Vec<String>) -> u64 {
    let mut group_set: HashSet<char> = HashSet::new();
    let mut set: HashSet<char> = HashSet::new();
    let mut to_remove: Vec<char> = Vec::new();

    let mut is_group_first = true;
    let mut count: u64 = 0;
    // probably there is a smart non for loop way of doing this that i am not aware of
    for line in lines {
        println!("{}", line);
        if !line.is_empty() {
            for char in line.chars() {
                set.insert(char);
                if is_group_first {
                    group_set.insert(char);
                }
            }

            for cc in group_set.iter() {
                if !set.contains(cc) {
                    to_remove.push(*cc);
                }
            }
            for tr in &to_remove {
                group_set.remove(&tr);
            }

            to_remove.clear();
            set.clear();
            is_group_first = false;
        }
        else {
            count += group_set.len() as u64;
            group_set.clear();
            is_group_first = true;
        }
    }

    count += group_set.len() as u64;
    count
}


pub(crate) fn run() {
    let lines = get_lines("./input/6in.txt").unwrap();

    let answer1 = solution(&lines);
    println!("\nday 6, - answer 1: {}", answer1);

    let answer2 = solution2(&lines);
    println!("\nday 6, - answer 2: {}", answer2);
}
