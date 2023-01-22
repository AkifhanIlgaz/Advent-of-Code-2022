use std::fs;

fn main() {
    let text = fs::read_to_string("./input.txt").unwrap();

    let mut point: u32 = 0;

    for round in text.trim_end().lines() {
        let round: Vec<char> = round.chars().collect();
        let opponent = shape_to_point(round[0]);
        let strategy = round[2];

        point += calc_point(opponent, strategy);
    }

    println!("{}", point)
}

fn shape_to_point(shape: char) -> u8 {
    shape as u8 - b'A' + 1
}

fn calc_point(opponent: u8, strategy: char) -> u32 {
    let mut my_move: u8;

    if strategy == 'X' {
        my_move = opponent - 1;
        if my_move == 0 {
            my_move = 3;
        }
    } else if strategy == 'Y' {
        my_move = opponent + 3
    } else {
        my_move = (opponent + 1) % 3;
        if my_move == 0 {
            my_move = 3
        }
        my_move += 6
    }

    my_move as u32
}
