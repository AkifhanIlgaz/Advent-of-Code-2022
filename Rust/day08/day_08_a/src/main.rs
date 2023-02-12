use std::fs;

struct Trees {
    grid: Vec<Vec<u8>>,
}

impl Trees {
    fn new(input: String) -> Trees {
        let mut grid: Vec<Vec<u8>> = vec![];

        for row in input.lines() {
            grid.push(row.as_bytes().to_vec());
        }

        Trees { grid }
    }

    fn is_visible(&self, i: usize, j: usize) -> bool {
        let l = self.grid[i][j];

        let mut visible = vec![true; 4]; // [left, right, top, bottom]

        if i == 0 || j == 0 || i == self.grid.len() - 1 || j == self.grid[0].len() - 1 {
            return true;
        }

        // Check left neighbors
        for x in 0..i {
            if self.grid[x][j] >= l {
                visible[0] = false;
                break;
            }
        }

        // Check right neighbors
        for x in i + 1..self.grid.len() {
            if self.grid[x][j] >= l {
                visible[1] = false;
                break;
            }
        }

        // Check top neighbors
        for y in 0..j {
            if self.grid[i][y] >= l {
                visible[2] = false;
                break;
            }
        }

        // Check bottom neighbors
        for y in j + 1..self.grid[0].len() {
            if self.grid[i][y] >= l {
                visible[3] = false;
                break;
            }
        }

        visible.contains(&true)
    }

    fn calc_visible_trees(&self) {
        let mut count = 0;

        for i in 0..self.grid.len() {
            for j in 0..self.grid[0].len() {
                if Self::is_visible(&self, i, j) {
                    count += 1;
                }
            }
        }

        println!("{}", count);
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let grid = Trees::new(input);
    grid.calc_visible_trees();
}
