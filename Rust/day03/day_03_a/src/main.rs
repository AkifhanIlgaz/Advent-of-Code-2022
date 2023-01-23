use std::fs;

fn main() {
    let text = fs::read_to_string("./input.txt").unwrap();

    let total_priority = text.lines().map(find_common).map(prioritize).sum::<u32>();

    // let point = text
    //     .lines()
    //     .map(|pack| pack.split_at(pack.len() / 2))
    //     .map(|(first, second)| first.chars().find(|ch| second.contains(*ch)).unwrap())
    //     .map(|snack| prioritize(snack))
    //     .sum::<u32>();

    println!("{}", total_priority);
}

fn prioritize(ch: char) -> u32 {
    if ch.is_ascii_lowercase() {
        return ((ch as u8 - b'a') + 1) as u32;
    } else {
        return ((ch as u8 - b'A') + 27) as u32;
    }
}

fn find_common(pack: &str) -> char {
    let (first, second) = pack.split_at(pack.len() / 2);

    first.chars().find(|ch| second.contains(*ch)).unwrap()
}
