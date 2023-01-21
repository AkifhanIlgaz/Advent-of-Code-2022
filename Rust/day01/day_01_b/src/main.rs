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

fn solution() {
    let text = fs::read_to_string("./input.txt").unwrap();

    let mut calories = vec![0];

    for cals in text.split("\n\n") {
        let sum = cals.lines().map(|c| c.parse::<u32>().unwrap()).sum::<u32>();
        for i in 0..calories.len() {
            if sum > calories[i] {
                calories.insert(i, sum);
                break;
            }
        }
    }

    println!("{:?}", calories.iter().take(3).sum::<u32>());
}
