use crate::files::{print_all, get_lines};
use std::collections::HashSet;
use regex::Regex;

fn contains_all(required: &Vec<String>, set: &HashSet<String>) -> bool {
    for field in required.iter() {
        if !set.contains(field) {
            println!("INVALID, 1st missing: {}  ,  set count: {}\n", field, set.len());
            return false;
        }
    }

    println!("valid!\n");
    return true;
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

    let mut field_set = HashSet::new();
    let mut valid_count = 0;

    for line in lines.iter().map(|l| l.as_str()) {
        println!("{}", line);
        if !line.is_empty() {
            for cap in regex.captures_iter(line) {
                let field_cap = cap[1].to_owned();
                field_set.insert(field_cap);
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

    println!("day 4 - VALID PASSPORT COUNT: {}", valid_count);
}
