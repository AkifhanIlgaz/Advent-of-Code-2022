use std::fs;

struct Cargoship {
    stacks: Vec<Vec<char>>,
}

impl Cargoship {
    fn new(structure: Vec<&str>) -> Self {
        let mut stacks: Vec<Vec<char>> = vec![vec![]; 10];

        for line in structure.iter().rev() {
            for (i, ch) in line.chars().skip(1).step_by(4).enumerate() {
                if ch != ' ' {
                    stacks[i + 1].push(ch);
                }
            }
        }

        Cargoship { stacks }
    }

    fn rearrange(&mut self, instructions: Vec<&str>) {
        for ins in instructions {
            let instruction = Instruction::from_str(ins);
            self.move_cranes(instruction);
        }
    }

    fn move_cranes(&mut self, instruction: Instruction) {
        for _ in 0..instruction.num_of_crates {
            let cra = self.stacks[instruction.src].pop().unwrap();
            self.stacks[instruction.dest].push(cra);
        }
    }

    fn get_top_of_each_stack(&self) -> String {
        let mut result = String::new();

        for stack in self.stacks.iter().skip(1) {
            result.push(*stack.last().unwrap());
        }

        result
    }
}

#[derive(Debug)]
struct Instruction {
    num_of_crates: u32,
    src: usize,
    dest: usize,
}

impl Instruction {
    fn from_str(input: &str) -> Instruction {
        let ins: Vec<u32> = input
            .split_whitespace()
            .filter_map(|a| a.parse::<u32>().ok())
            .collect();

        Instruction {
            num_of_crates: ins[0],
            src: ins[1] as usize,
            dest: ins[2] as usize,
        }
    }
}

fn main() {
    let text = fs::read_to_string("./input.txt").unwrap();

    let stack_structure: Vec<&str> = text.lines().take(8).collect();
    let instructions: Vec<&str> = text.lines().skip(10).collect();

    let mut cargo = Cargoship::new(stack_structure);
    cargo.rearrange(instructions);

    
    cargo.print_top_of_each_stack();
}
