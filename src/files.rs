use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_raw_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();
    return Ok(lines);
}

pub fn get_lines(path: &str) -> Option<Vec<String>> {
    let read_result = read_raw_lines(path);
    return match read_result {
        Ok(lines) => Option::from(lines
            .filter(|l| l.is_ok())
            .map(|r| r.unwrap())
            .collect::<Vec<String>>()),
        // instead of this, maybe panic / err here ?
        Err(_e) => None
    }
}

pub(crate) fn print_all(lines: &Vec<String>) {
    for l in lines {
        println!("{}", l);
    }
}