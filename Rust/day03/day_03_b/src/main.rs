use std::fs;

fn main() {
    let text = fs::read_to_string("./input.txt").unwrap();

    let mut total_priority = 0;
    let elves: Vec<&str> = text.lines().collect();

    for group in elves.chunks(3) {
        let badge = find_badge(group);
        total_priority += prioritize(badge);
    }

    // let mut total_priority = elves.chunks(3).map(find_badge).map(prioritize).sum::<u32>();

    println!("{}", total_priority);
}

fn prioritize(ch: char) -> u32 {
    if ch.is_ascii_lowercase() {
        return ((ch as u8 - b'a') + 1) as u32;
    } else {
        return ((ch as u8 - b'A') + 27) as u32;
    }
}

fn find_badge(group: &[&str]) -> char {
    group[0]
        .chars()
        .find(|&item| group[1].contains(item) && group[2].contains(item))
        .unwrap()
}
