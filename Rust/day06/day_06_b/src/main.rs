use std::fs;

fn main() {
    let text = fs::read_to_string("./input.txt").unwrap();

    let data_stream = text.as_bytes();

    find_first_start_of_message_marker(data_stream, 14);
}

fn find_first_start_of_message_marker(data_stream: &[u8], num_of_distinct_characters: usize) {
    for (i, chunk) in data_stream.windows(num_of_distinct_characters).enumerate() {
        if is_all_unique(chunk, num_of_distinct_characters) {
            println!("{}", i + num_of_distinct_characters);
            break;
        }
    }
}

fn is_all_unique(chunk: &[u8], num_of_distinct_characters: usize) -> bool {
    let mut unique_chars = vec![];

    for ch in chunk {
        if !unique_chars.contains(ch) {
            unique_chars.push(*ch);
        }
    }
    unique_chars.len() == num_of_distinct_characters
}
