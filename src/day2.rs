use crate::files;

struct Policy {
    pub letter: char,
    pub min: usize,
    pub max: usize
}

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

fn solution(lines: Vec<String>) {
    let mut valid_count = 0;
    for line in lines {
        // wanted this to be a function, but had some difficulty with lifetime specification
        let policy_pair = line.split(":")
                                        .map(str::trim)
                                        .collect::<Vec<&str>>();

        // real code would handle errors in parsing
        let policy = Policy::parse(policy_pair[0]);
        // use 'validate' to answer puzzle 1
        if policy.validate2(policy_pair[1]) {
            valid_count += 1;
        }
    }

    println!("day 2 - VALID COUNT: {}", valid_count);
}

pub(crate) fn run() {
    solution(files::get_lines("./input/2in.txt").unwrap());
}
