use std::{collections::HashSet, fs};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn origin() -> Self {
        Self { x: 0, y: 0 }
    }

    // move 1 step
    fn r#move(&mut self, direction: &str) {
        match direction {
            "R" => self.x += 1,
            "L" => self.x -= 1,
            "D" => self.y -= 1,
            "U" => self.y += 1,
            _ => unreachable!(),
        }
    }
}

// Coordinate system
struct Grid {
    head: Point,
    tail: Point,
    visited_positions: HashSet<Point>,
}

impl Grid {
    fn init() -> Self {
        let mut visited_positions = HashSet::new();
        visited_positions.insert(Point::origin());

        Self {
            head: Point::origin(),
            tail: Point::origin(),
            visited_positions,
        }
    }

    fn move_head(&mut self, direction: &str) {
        self.head.r#move(direction);
    }

    fn move_tail(&mut self) {
        //vertical
        if self.head.x == self.tail.x {
            if self.head.y > self.tail.y {
                while !self.is_touching() {
                    self.tail.r#move("U");
                }
            } else {
                while !self.is_touching() {
                    self.tail.r#move("D");
                }
            }

            self.visited_positions.insert(self.tail.clone());
        }

        // horizontal
        if self.head.y == self.tail.y {
            if self.head.x > self.tail.x {
                while !self.is_touching() {
                    self.tail.r#move("R");
                }
            } else {
                while !self.is_touching() {
                    self.tail.r#move("L");
                }
            }
            self.visited_positions.insert(self.tail.clone());
        }

        // move diagonal
        while !self.is_touching() {
            if self.head.x > self.tail.x && self.head.y > self.tail.y {
                self.tail.r#move("R");
                self.tail.r#move("U");
            } else if self.head.x > self.tail.x && self.head.y < self.tail.y {
                self.tail.r#move("D");
                self.tail.r#move("R");
            } else if self.head.x < self.tail.x && self.head.y < self.tail.y {
                self.tail.r#move("D");
                self.tail.r#move("L");
            } else if self.head.x < self.tail.x && self.head.y > self.tail.y {
                self.tail.r#move("U");
                self.tail.r#move("L");
            }
            self.visited_positions.insert(self.tail.clone());
        }
    }

    fn is_touching(&self) -> bool {
        let temp = [-1, 0, 1];

        temp.contains(&(self.head.x - self.tail.x)) && temp.contains(&(self.head.y - self.tail.y))
    }

    fn execute_motion(&mut self, motion: &str) {
        let m: Vec<&str> = motion.split_ascii_whitespace().collect();
        let direction = m[0];
        let steps = m[1].parse::<i32>().unwrap();
        for _ in 0..steps {
            self.move_head(direction);
            if !self.is_touching() {
                self.move_tail()
            }
        }
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let mut grid = Grid::init();

    for motion in input.lines() {
        grid.execute_motion(motion);
    }

    println!("{:?}", grid.visited_positions.len());
}
