use crate::instruction::Instruction;

pub struct Cargoship {
    stacks: Vec<Vec<char>>,
}

impl Cargoship {
    pub fn new(structure: Vec<&str>) -> Self {
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

    pub fn rearrange(&mut self, instructions: Vec<&str>) {
        for ins in instructions {
            let instruction = Instruction::from_str(ins);
            self.move_cranes(instruction);
        }
    }

    pub fn move_cranes(&mut self, instruction: Instruction) {
        let src_len = self.stacks[instruction.src].len();

        let mut temp = vec![];

        self.stacks[instruction.src]
            .drain((src_len - instruction.num_of_crates as usize)..)
            .for_each(|crane| temp.push(crane));

        self.stacks[instruction.dest].append(&mut temp);
    }

    pub fn print_top_of_each_stack(&self) {
        for stack in self.stacks.iter().skip(1) {
            print!("{}", *stack.last().unwrap());
        }
    }
}
