use cargoship::Cargoship;
use std::fs;

mod cargoship;
mod instruction;

fn main() {
    let text = fs::read_to_string("./input.txt").unwrap();

    let stack_structure: Vec<&str> = text.lines().take(8).collect();
    let instructions: Vec<&str> = text.lines().skip(10).collect();

    let mut cargo = Cargoship::new(stack_structure);
    cargo.rearrange(instructions);

    cargo.print_top_of_each_stack();
}
