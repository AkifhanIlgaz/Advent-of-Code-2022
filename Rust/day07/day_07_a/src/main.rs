use std::{collections::HashMap, fs};

struct Directory {
    dirs: Vec<String>,
    files: Vec<String>,
}

impl Directory {
    fn new() -> Self {
        Directory {
            dirs: vec![],
            files: vec![],
        }
    }
}

fn main() {
    let text = fs::read_to_string("./input.txt").unwrap();

    let mut stack = vec![];
    let mut directories = HashMap::new();

    let mut current_directory = &mut Directory::new();

    for (i, command) in text.lines().enumerate() {
        if command.starts_with("$ cd") {
            let c = command.strip_prefix("$ cd ").unwrap();
            if c == ".." {
                stack.pop();
                return;
            } else {
                directories.entry(c.to_string()).or_insert(Directory::new());
                stack.push(c.to_string());
            }
        } else if command == "$ ls" {
            // $ ls
            let current_dir = stack.last().unwrap();
            current_directory = directories.get_mut(current_dir).unwrap();
        } else {
            if command.starts_with("dir") {
                current_directory
                    .dirs
                    .push(command.strip_prefix("dir ").unwrap().to_string());
            } else {
            }
        }
    }
}

fn execute_command(
    command: &str,
    stack: &mut Vec<String>,
    directories: &mut HashMap<String, Directory>,
) {
    if command.starts_with("$ cd") {
        let c = command.strip_prefix("$ cd ").unwrap();
        if c == ".." {
            stack.pop();
            return;
        } else {
            directories.entry(c.to_string()).or_insert(Directory::new());
            stack.push(c.to_string());
        }
    } else { // $ ls
    }
}

// fn find_first_start_of_packet_marker(data_stream: &[u8], num_of_distinct_characters: usize) {
//     for (i, chunk) in data_stream.windows(num_of_distinct_characters).enumerate() {
//         if is_all_unique(chunk, num_of_distinct_characters) {
//             println!("{}", i + num_of_distinct_characters);
//             break;
//         }
//     }
// }

// fn is_all_unique(chunk: &[u8], num_of_distinct_characters: usize) -> bool {
//     let mut unique_chars = vec![];

//     for ch in chunk {
//         if !unique_chars.contains(ch) {
//             unique_chars.push(*ch);
//         }
//     }
//     unique_chars.len() == num_of_distinct_characters
// }
