use crate::files::{print_all, get_lines};
use std::str::FromStr;
use std::collections::HashSet;

struct Seat {
    row: i32,
    column: i32
}

impl Seat {
    fn get_id(&self) -> i32 { &self.row * 8 + &self.column }
}

struct BspEntry {
    pub moves: Vec<char>
}
impl BspEntry {
    fn do_moves(&self) -> Seat {
        let mut v_bounds = (0, 127);
        let mut h_bounds = (0, 7);
        for m in &self.moves {
            match m {
                'F' => { v_bounds.1 = (&v_bounds.0 + v_bounds.1) / 2 },
                'B' => { v_bounds.0 = ((v_bounds.0 + &v_bounds.1) / 2) + 1 },
                'L' => { h_bounds.1 = (&h_bounds.0 + h_bounds.1) / 2 },
                'R' => { h_bounds.0 = ((h_bounds.0 + &h_bounds.1) / 2) + 1 },
                &_ => {}
            }
        }
        Seat { row: v_bounds.0, column: h_bounds.0 }
    }
}

impl FromStr for BspEntry {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut moves: Vec<char> = Vec::new();
        for c in s.chars() {
            moves.push(c);
        }
        Ok(BspEntry { moves })
    }
}

fn solution(lines: &Vec<String>) -> i32 {
    let mut max_seat_id = 0;
    for line in lines {
        let entry = BspEntry::from_str(line).unwrap();
        let seat = entry.do_moves();
        let seat_id = seat.get_id();
        //println! ("seat id: {}", seat_id);
        if seat_id > max_seat_id {
            max_seat_id = seat_id;
        }
    }
    return max_seat_id;
}

fn solution2(lines: &Vec<String>) -> i32 {
    let mut max_seat_id = 0;
    let mut id_set = HashSet::new();
    for line in lines {
        let entry = BspEntry::from_str(line).unwrap();
        let seat = entry.do_moves();
        let seat_id = seat.get_id();
        id_set.insert(seat_id);
        if seat_id > max_seat_id {
            max_seat_id = seat_id;
        }
    }

    for i in 0..max_seat_id {
        if !id_set.contains(&i) {
            // the right answer is the last one it prints.
            println! ("id not in set: {}", i);
        }
    }

    return 0;
}


pub(crate) fn run() {
    let lines = get_lines("./input/5in.txt").unwrap();

    let highest_id = solution(&lines);
    println!("\nday 5, - highest seat id: {}", highest_id);

    solution2(&lines);
}
