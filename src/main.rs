use std::fs::File;
use std::io::{self, BufRead, Lines, BufReader};
use std::path::Path;
use std::iter::Filter;

// TODO - move into utils file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();
    return Ok(lines);
}

// TODO - move into utils file
fn load_file(path: &str) -> Option<Vec<String>> {
    let read_result = read_lines(path);
    return match read_result {
        Ok(lines) => Option::from(lines
                                                .filter(|l| l.is_ok())
                                                .map(|r| r.unwrap())
                                                .collect::<Vec<String>>()),
        Err(_e) => None
    }
}

struct Policy {
    pub letter: char,
    pub min: usize,           // these names only make
    pub max: usize            // sense in the 1st puzzle
}

// all `Policy` methods go in here
impl Policy {
    fn parse(s: &str) -> Policy {
        // this string splitting code is nasty,
        // probably there is a smarter way to do this via iterating over it or regex?
        let left = s.split("-").collect::<Vec<&str>>();
        let right = left[1].split(" ").collect::<Vec<&str>>();
        let max_str = right[0];
        let char_str = right[1];

        Policy {
            letter: char_str.chars().collect::<Vec<char>>()[0],
            min: left[0].parse::<usize>().unwrap(),
            max: max_str.parse::<usize>().unwrap()
        }
    }

    // first puzzle solution
    fn validate(&self, pw: &str) -> bool {
        let mut count = 0;
        for c in pw.chars() {
            if c == self.letter {
                count += 1;
            }
        }

        return count >= self.min && count <= self.max;
    }

    // second puzzle solution
    fn validate2(&self, pw: &str) -> bool {
        let mut match_cnt = 0;
        for (i, c) in pw.chars().enumerate() {
            let i1 = i + 1;
            let idx_matches = i1 == self.min || i1 == self.max;
            let char_matches = c == self.letter;
            if idx_matches && char_matches {
                match_cnt += 1;
            }
        }

        return match_cnt == 1;
    }

    fn to_string(&self) -> String {
        return format!("char: {}, {}-{}", self.letter, self.min, self.max);
    }
}

// first puzzle solution
fn solution(lines: Vec<String>) {
    // TODO - cleanup parsing
    let mut valid_count = 0;
    for line in lines {
        let split = line.split(":")
                                    .map(str::trim)
                                    .collect::<Vec<&str>>();

        // real code would handle errors in parsing
        let policy = Policy::parse(split[0]);
        // use 'validate' to answer puzzle 1
        if policy.validate2(split[1]) {
            valid_count += 1;
        }
    }

    println!("VALID COUNT: {}", valid_count);
}

fn main() {
    let lines_opt = load_file("./input/2in.txt").unwrap();

    solution(lines_opt);
}
