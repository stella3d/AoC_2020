use crate::files::{print_all, get_lines};
use std::str::FromStr;
use std::collections::HashSet;
use regex::{Regex, Captures};

struct ContentRule {
    type_index: usize,
    count: u32
}

struct BagRuleSet {
    name: String,
    type_index: usize,
    rules: Vec<ContentRule>
}

lazy_static! {
    static ref BAG_TYPE_RGX: Regex = Regex::new(r"^(\w+ \w+) bags").unwrap();
    static ref CONTENT_RULE_RGX: Regex = Regex::new(r"(\d+) (\w+ \w+) bags").unwrap();
}


// NOT FINISHED FUCK THIS PUZZLE
fn solution(lines: &Vec<String>) -> u64 {
    let mut types: Vec<BagRuleSet> = Vec::new();
    let mut search_set: HashSet<String> = HashSet::new();
    let mut count: u64 = 0;

    for line in lines {
        println!("{}", line);
        match BAG_TYPE_RGX.captures(line) {
            None => {},
            Some(caps) => {
                println!("bag type: {}", &caps[1]);

                for rule_caps in CONTENT_RULE_RGX.captures_iter(line) {
                    for rc in rule_caps.iter() {
                        match rc {
                            None => {},
                            Some(rc) => {
                                println!("    rule: {}", &rc.as_str());
                            }
                        }
                    }
                }
            }
        }
    }

    count
}

fn solution2(lines: &Vec<String>) -> u64 {
    0
}


pub(crate) fn run() {
    let lines = get_lines("./input/7in.txt").unwrap();

    let answer1 = solution(&lines);
    println!("\nday 7, - answer 1: {}", answer1);

    //let answer2 = solution2(&lines);
    //println!("\nday 7, - answer 2: {}", answer2);
}
