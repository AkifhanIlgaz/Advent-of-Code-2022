use std::fs;

struct Range {
    start: i32,
    end: i32,
}

impl Range {
    fn from_str(s: &str) -> Self {
        let (start, end) = s.split_once("-").unwrap();
        let start = start.parse::<i32>().unwrap();
        let end = end.parse::<i32>().unwrap();

        Range { start, end }
    }
}

fn main() {
    let text = fs::read_to_string("./input.txt").unwrap();

    let mut total = 0;

    for pair in text.lines() {
        let (first, second) = pair.split_once(",").unwrap();
        let first = Range::from_str(first);
        let second = Range::from_str(second);

        if is_subset(&first, &second) {
            total += 1;
        }
    }

    println!("{}", total);
}

fn is_subset(first: &Range, second: &Range) -> bool {
    (first.start >= second.start && first.end <= second.end)
        || (second.start >= first.start && second.end <= first.end)
}
