use std::{collections::HashSet, fs};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let mut visited_by_tail = HashSet::new();
    let mut knots: [Point; 10] = [Point { x: 0, y: 0 }; 10];
    visited_by_tail.insert(knots[9]);

    for instruction in input.lines() {
        let instruction: Vec<&str> = instruction.split_ascii_whitespace().collect();
        let direction = instruction[0];
        let mut steps = instruction[1].parse::<i32>().unwrap();

        while steps > 0 {
            match direction {
                "U" => knots[0].y += 1,
                "R" => knots[0].x += 1,
                "D" => knots[0].y -= 1,
                "L" => knots[0].x -= 1,
                _ => unreachable!(),
            }

            for i in 0..(knots.len() - 1) {
                knots[i + 1] = adjust_tail(knots[i + 1], knots[i]);
            }
            steps -= 1;
            visited_by_tail.insert(knots[9]);
        }
    }

    println!("{}", visited_by_tail.len())
}

fn adjust_tail(tail: Point, head: Point) -> Point {
    let mut new_tail = tail;

    match (head.x - tail.x, head.y - tail.y) {
        (-2, 1) | (-1, 2) | (0, 2) | (1, 2) | (2, 1) | (2, 2) | (-2, 2) => new_tail.y += 1,
        _ => (),
    }

    match (head.x - tail.x, head.y - tail.y) {
        (1, 2) | (2, 1) | (2, 0) | (2, -1) | (1, -2) | (2, 2) | (2, -2) => new_tail.x += 1,
        _ => (),
    }

    match (head.x - tail.x, head.y - tail.y) {
        (-2, -2) | (2, -1) | (1, -2) | (0, -2) | (-1, -2) | (-2, -1) | (2, -2) => new_tail.y -= 1,
        _ => (),
    }

    match (head.x - tail.x, head.y - tail.y) {
        (-2, -2) | (-1, -2) | (-2, -1) | (-2, -0) | (-2, 1) | (-1, 2) | (-2, 2) => new_tail.x -= 1,
        _ => (),
    }

    new_tail
}
