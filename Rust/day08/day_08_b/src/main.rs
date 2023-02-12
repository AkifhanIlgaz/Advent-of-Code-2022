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

    fn scenic_score(&self, i: usize, j: usize) -> u32 {
        let l = self.grid[i][j];

        let mut visible = vec![0; 4]; // [left, right, top, bottom]

        // Edges
        if i == 0 || j == 0 || i == self.grid.len() - 1 || j == self.grid[0].len() - 1 {
            return 0;
        }

        // Check left neighbors
        for x in (0..i).rev() {
            if self.grid[x][j] >= l {
                visible[0] += 1;
                break;
            }
            visible[0] += 1;
        }

        // Check right neighbors
        for x in i + 1..self.grid.len() {
            if self.grid[x][j] >= l {
                visible[1] += 1;
                break;
            }
            visible[1] += 1;
        }

        // Check top neighbors
        for y in (0..j).rev() {
            if self.grid[i][y] >= l {
                visible[2] += 1;
                break;
            }
            visible[2] += 1;
        }

        // Check bottom neighbors
        for y in j + 1..self.grid[0].len() {
            if self.grid[i][y] >= l {
                visible[3] += 1;
                break;
            }
            visible[3] += 1;
        }

        visible.iter().fold(1, |acc, x| acc * *x)
    }

    fn calc_max_scenic_score(&self) {
        let mut max_score = 0;

        for i in 0..self.grid.len() {
            for j in 0..self.grid[0].len() {
                max_score = max_score.max(Self::scenic_score(&self, i, j));
            }
        }

        println!("{}", max_score);
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let grid = Trees::new(input);
    grid.calc_max_scenic_score();
}
