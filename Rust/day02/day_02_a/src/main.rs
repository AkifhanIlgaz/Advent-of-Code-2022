use std::fs;

fn main() {
    let text = fs::read_to_string("./input.txt").unwrap();

    let mut point = 0;

    for round in text.lines() {
        let r: Vec<&str> = round.split_whitespace().collect();
        let opponent = shape_point(r[0]);
        let me = shape_point(r[1]);

        point += calc_point(opponent, me);
    }

    println!("{}", point);
}

fn calc_point(opponent: i32, me: i32) -> i32 {
    let mut point = 0;
    let diff = (opponent - me).abs();

    if opponent == me {
        // Draw
        point += 3
    } else if (diff == 1 && me > opponent) || (me == 1 && opponent == 3) {
        // Win
        point += 6;
    }
    point + me
}

fn shape_point(sh: &str) -> i32 {
    match sh {
        "A" | "X" => 1,
        "B" | "Y" => 2,
        "C" | "Z" => 3,
        _ => unreachable!(),
    }
}
