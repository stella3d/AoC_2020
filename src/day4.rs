use crate::files::{print_all, get_lines};
use std::collections::HashSet;
use regex::{Regex, Captures};
// rust's default hash algo is slower than ideal, for web security reasons - use a fast one
use fnv::FnvHashSet;

fn contains_all(required: &Vec<String>, set: &FnvHashSet<String>,) -> bool {
    for field in required.iter() {
        if !set.contains(field) {
            return false;
        }
    }
    return true;
}

fn check_field(name: &str, value: &str) -> bool {
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
                static ref COLOR_REGEX: Regex = Regex::new("^#[a-z0-9]{6}$").unwrap();
            }
            COLOR_REGEX.is_match(value)
        },
        "ecl" => {
            lazy_static! {
                static ref EYE_COLOR_SET: HashSet<&'static str> =
                    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter().cloned().collect();
            }
            EYE_COLOR_SET.contains(value)
        },
        "pid" => {
            lazy_static! {
                static ref PID_REGEX: Regex = Regex::new("^[0-9]{9}$").unwrap();
            }
            PID_REGEX.is_match(value)
        },
        &_ => { true }
    };
}

fn add_field_if_valid(field_set: &mut FnvHashSet<String>, cap: Captures)  {
    let field_cap = cap[1].to_owned();
    let value_cap = cap[2].to_owned();
    if check_field(&field_cap, &value_cap) {
        field_set.insert(field_cap);
    }
}

fn solution(lines: &Vec<String>, required_fields: &Vec<String>, regex: &Regex) -> i32 {
    let mut field_set = FnvHashSet::with_capacity_and_hasher(8, Default::default());

    let mut valid_count = 0;
    for line in lines.iter().map(|l| l.as_str()) {
        if !line.is_empty() {
            for cap in regex.captures_iter(line) {
                // difference between part 1 & 2 is part 2 validates fields before adding to set
                // field_set.insert(cap[1].to_owned());             // part 1
                add_field_if_valid(&mut field_set, cap);            // part 2
            }
        }
        else {
            if contains_all(&required_fields, &field_set) {
                valid_count += 1;
            }
            field_set.clear();
        }
    }

    if contains_all(&required_fields, &field_set) { valid_count += 1; }
    return valid_count;
}

pub(crate) fn run() {
    let lines = get_lines("./input/4in.txt").unwrap();
    // 1st capture group captures the field name, 2nd group gets the value
    let field_rgx = Regex::new(r"([a-z]{3}):([a-z0-9#]*)").unwrap();
    // wanted to make this a static but had trouble creating it as 'String' instead of '&str'
    let required_fields = vec![
        String::from("byr"),
        String::from("iyr"),
        String::from("eyr"),
        String::from("hgt"),
        String::from("hcl"),
        String::from("ecl"),
        String::from("pid")
    ];

    let valid_count = solution(&lines, &required_fields, &field_rgx);
    println!("\nday 4, - VALID COUNT: {}", valid_count);
}
