use std::fs::File;
use std::io::{self, BufRead, Lines, BufReader};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn load_file(path: &str) -> Option<Lines<BufReader<File>>> {
    let read_result = read_lines(path);
    return match read_result {
        Ok(lines) => Option::from(lines),
        Err(_e) => None
    }
}

// first puzzle solution
fn solution1(lines: Lines<BufReader<File>>) {
    let mut vec: Vec<i32> = Vec::new();

    for line in lines {
        // this could be replace with a filter_map maybe ?
        if let Ok(ip) = line {
            vec.push(ip.parse::<i32>().unwrap());
        }
    }

    let end = vec.len() - 1;
    for (li, l) in vec.iter().enumerate() {
        let rs = li + 1 as usize;
        for ri in rs..end {
            let r = vec[ri];
            let sum = l + r;
            if sum == 2020 {
                let result = l * r;
                println!("RESULT: {}", result);
            }
        }
    }
}

// second puzzle solution
fn solution2(lines: Lines<BufReader<File>>) {
    let mut vec: Vec<i32> = Vec::new();

    for line in lines {
        if let Ok(ip) = line {
            vec.push(ip.parse::<i32>().unwrap());
        }
    }

    // don't do this brute force bullshit, use the two-pointers algorithm or hashing method:
    // https://www.geeksforgeeks.org/find-a-triplet-that-sum-to-a-given-value/
    let end = vec.len() - 1;
    for (li, l) in vec.iter().enumerate() {
        let ms = li + 1 as usize;
        let rs = li + 2 as usize;
        for mi in ms..end {
            let m = vec[mi];
            for ri in rs..end {
                let r = vec[ri];
                let sum = l + r + m;
                if sum == 2020 {
                    let result = l * r * m;
                    println!("RESULT: {}", result);
                }
            }
        }
    }
}

fn main() {
    println!("Hello, world!");

    // had some trouble with match statement here so just defaulted to .unwrap()
    let lines_opt = load_file("./input/1in.txt").unwrap();

    //process(lines_opt);
    solution2(lines_opt);
}
