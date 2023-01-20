use std::fs;

fn main() {
    let text = fs::read_to_string("./input.txt").unwrap();

    let mut calories: Vec<u32> = text
        .split("\n\n")
        .map(|cals| cals.lines().map(|c| c.parse::<u32>().unwrap()).sum::<u32>())
        .collect();

    calories.sort_by(|a, b| b.cmp(&a));

    let sum_of_most_three = calories.iter().take(3).sum::<u32>();

    println!("{}", sum_of_most_three);
}
