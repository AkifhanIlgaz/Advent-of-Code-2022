pub struct Instruction {
    pub num_of_crates: u32,
    pub src: usize,
    pub dest: usize,
}

impl Instruction {
    pub fn from_str(input: &str) -> Instruction {
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
