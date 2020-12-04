use crate::files::{print_all, get_lines};
use std::collections::HashSet;
use regex::Regex;

fn contains_all(required: &Vec<String>, set: &HashSet<String>) -> bool {
    for field in required.iter() {
        if !set.contains(field) {
            return false;
        }
    }
    return true;
}

fn first_solution(lines: &Vec<String>, required_fields: &Vec<String>, regex: &Regex) -> i32 {
    let mut field_set = HashSet::new();
    let mut valid_count = 0;

    for line in lines.iter().map(|l| l.as_str()) {
        if !line.is_empty() {
            for cap in regex.captures_iter(line) {
                field_set.insert(cap[1].to_owned());
            }
        }
        else {
            if contains_all(&required_fields, &field_set) {
                valid_count += 1;
            }
            field_set.clear();
        }
    }

    // since there's no blank line at the end of the file, we'll be off by -1
    // if this check is not included after the loop
    if contains_all(&required_fields, &field_set) {
        valid_count += 1;
    }

    return valid_count;
}

fn check_field(name: &String, value: &String) -> bool {
    return match &name[..] {
        "byr" => {
            let n = value.parse::<i32>().unwrap();
            1920 <= n && n <= 2002
        },
        "iyr" => {
            let n = value.parse::<i32>().unwrap();
            2010 <= n && n <= 2020
        },
        "eyr" => {
            let n = value.parse::<i32>().unwrap();
            2020 <= n && n <= 2030
        },
        "hgt" => {
            let mut result = false;
            // TODO - function for this check ?
            if value.ends_with("cm") {
                let num_str = value.replace("cm", "");
                let n = num_str.parse::<i32>().unwrap();
                result = 150 <= n && n <= 193;
            }
            else if value.ends_with("in") {
                let num_str = value.replace("in", "");
                let n = num_str.parse::<i32>().unwrap();
                result = 59 <= n && n <= 76;
            }
            result
        },
        "hcl" => {
            // why is this the recommended pattern, wtf.  Rust get real statics please
            lazy_static! {
                static ref color_rgx: Regex = Regex::new("^#[a-z0-9]{6}$").unwrap();
            }
            color_rgx.is_match(value)
        },
        "ecl" => {
            lazy_static! {
                static ref eye_color_set: HashSet<&'static str> =
                    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter().cloned().collect();
            }
            eye_color_set.contains(value.as_str())
        },
        "pid" => {
            lazy_static! {
                static ref pid_rgx: Regex = Regex::new("^[0-9]{9}$").unwrap();
            }
            pid_rgx.is_match(value)
        },
        &_ => { true }
    };
}

// TODO - combine parts of solutions that are the same
fn second_solution(lines: &Vec<String>, required_fields: &Vec<String>, regex: &Regex) -> i32 {
    let mut field_set = HashSet::new();
    let mut valid_count = 0;

    for line in lines.iter().map(|l| l.as_str()) {
        if !line.is_empty() {
            for cap in regex.captures_iter(line) {
                let field_cap = cap[1].to_owned();
                let value_cap = cap[2].to_owned();
                if check_field(&field_cap, &value_cap) {
                    field_set.insert(field_cap);
                }
            }
        }
        else {
            if contains_all(&required_fields, &field_set) {
                valid_count += 1;
            }
            field_set.clear();
        }
    }

    if contains_all(&required_fields, &field_set) {
        valid_count += 1;
    }
    return valid_count;
}

pub(crate) fn run() {
    let lines = get_lines("./input/4in.txt").unwrap();
    //print_all(&lines);
    // single capture group captures the field name
    let regex = Regex::new(r"([a-z]{3}):([a-z0-9#]*)").unwrap();
    // wanted to make this a static but had trouble creating it as 'String' instead of '&str'
    let required_fields: Vec<String> = vec![
        String::from("byr"),
        String::from("iyr"),
        String::from("eyr"),
        String::from("hgt"),
        String::from("hcl"),
        String::from("ecl"),
        String::from("pid")
    ];

    let valid_count1 = first_solution(&lines, &required_fields, &regex);
    println!("\nday 4, part 1 - VALID COUNT: {}", valid_count1);

    let valid_count2 = second_solution(&lines, &required_fields, &regex);
    println!("\nday 4, part 2 - VALID COUNT: {}", valid_count2);
}
