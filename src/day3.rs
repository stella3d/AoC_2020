use crate::files;
use crate::files::{print_all, get_lines};

struct Terrain {
    pub pattern_width: usize,
    pub data: Vec<Vec<u8>>
}

impl Terrain {
    fn tree_at(&self, row: usize, column: usize) -> bool {
        return (&self.data[row])[column % self.pattern_width] == 35;    // # is 35 in ascii
    }

    fn from_strings(vs: &Vec<String>) -> Terrain {
        let first = vs[0].chars().collect::<Vec<char>>();
        Terrain {
            pattern_width: first.len(),
            data: vs.iter().map(|s| s.as_bytes().to_vec()).collect::<Vec<Vec<u8>>>()
        }
    }

    fn solve(&self, right_rate: usize, down_rate: usize) -> i64 {
        let mut count = 0;
        let mut column = 0;
        let mut row = 0;
        while row < self.data.len() {
            if self.tree_at(row, column) {
                count += 1;
            }
            row += down_rate;
            column += right_rate;
        }

        println!("right {}, down {}, count {}", right_rate, down_rate, count);
        return count;
    }
}

pub(crate) fn run() {
    let lines = files::get_lines("./input/3in.txt").unwrap();
    let terrain = Terrain::from_strings(&lines);

    let slopes = vec![(1,1), (3,1), (5,1), (7,1), (1,2)];
    let product = slopes.iter()
                            .map(|s| terrain.solve(s.0, s.1))
                            .fold(1, |acc, x| acc * x);

    println!("day 3 - MULTIPLIED TREE COUNT: {}", product);
}
