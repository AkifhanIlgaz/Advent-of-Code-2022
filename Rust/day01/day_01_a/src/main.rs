use std::fs;

fn main() {
    let text = fs::read_to_string("./input.txt").unwrap();

    let result = text
        .split("\n\n") // Each line has \n at the end. Empty line which is "\n".So \n\n pattern separates the elves
        .map(|cals| {
            cals.lines()
                .map(|cal| cal.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();

    println!("{}", result);
}


