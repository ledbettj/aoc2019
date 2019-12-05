use std::num::ParseIntError;

const INPUT : &'static str  = include_str!("../inputs/day5.txt");

struct Instruction {
    opcode: usize,
    modes:  Vec<usize>
}

struct Program {
    bytes: Vec<isize>
}

impl Program {
    pub fn parse(input: &str) -> Result<Program, ParseIntError> {
        let p = input
            .trim()
            .split(",")
            .map(|part| part.trim().parse::<isize>())
            .collect::<Result<Vec<isize>, ParseIntError>>()?;
        Ok(Program { bytes: p })
    }

    pub fn execute(&mut self, input: isize) {
        let mut i = 0;

        while i < self.bytes.len() {
            let instr = self.parse_instruction(self.bytes[i] as usize);
            match instr.opcode {
                // add
                1 => {
                    let v1 = self.load_argument(self.bytes[i + 1], instr.modes[0]);
                    let v2 = self.load_argument(self.bytes[i + 2], instr.modes[1]);
                    let v3 = self.bytes[i + 3];
                    println!("storing {} + {} at index {}", v1, v2, v3);
                    self.bytes[v3 as usize] = v1 + v2;
                    i += 4;
                },
                // mul
                2 => {
                    let v1 = self.load_argument(self.bytes[i + 1], instr.modes[0]);
                    let v2 = self.load_argument(self.bytes[i + 2], instr.modes[1]);
                    let v3 = self.bytes[i + 3];
                    println!("storing {} * {} at index {}", v1, v2, v3);
                    self.bytes[v3 as usize] = v1 * v2;
                    i += 4;
                },
                // input
                3 => {
                    let v1 = self.bytes[i + 1];
                    println!("providing {} to input at index {}", input, v1);
                    self.bytes[v1 as usize] = input;
                    i += 2;
                },
                // output
                4 => {
                    let v1 = self.load_argument(self.bytes[i + 1], instr.modes[0]);
                    println!("output: {}", v1);
                    i += 2;
                },
                5 => {
                    let v1 = self.load_argument(self.bytes[i + 1], instr.modes[0]);
                    let v2 = self.load_argument(self.bytes[i + 2], instr.modes[1]);
                    if v1 != 0 {
                        i = v2 as usize;
                    } else {
                        i += 3;
                    }
                },
                6 => {
                    let v1 = self.load_argument(self.bytes[i + 1], instr.modes[0]);
                    let v2 = self.load_argument(self.bytes[i + 2], instr.modes[1]);
                    if v1 == 0 {
                        i = v2 as usize;
                    } else {
                        i += 3;
                    }
                },
                7 => {
                    let v1 = self.load_argument(self.bytes[i + 1], instr.modes[0]);
                    let v2 = self.load_argument(self.bytes[i + 2], instr.modes[1]);
                    let v3 = self.bytes[i + 3];
                    self.bytes[v3 as usize] = if v1 < v2 { 1 } else { 0 };
                    i += 4;
                },
                8 => {
                    let v1 = self.load_argument(self.bytes[i + 1], instr.modes[0]);
                    let v2 = self.load_argument(self.bytes[i + 2], instr.modes[1]);
                    let v3 = self.bytes[i + 3];
                    self.bytes[v3 as usize] = if v1 == v2 { 1 } else { 0 };
                    i += 4;
                }
                99 => break,
                d => { panic!("Illegal instruction: {}", d) }
            };
        }
    }

    pub fn load_argument(&self, value: isize, mode: usize) -> isize {
        match mode {
            0 => self.bytes[value as usize],
            1 => value,
            n => panic!("Illegal argument mode: {}", n)
        }
    }

    pub fn parse_instruction(&self, opcode: usize) -> Instruction  {
        let instr = opcode % 100;
        let modes = vec![
            (opcode / 100)   % 10,
            (opcode / 1000)  % 10,
            (opcode / 10000) % 10,
        ];
        println!("instr = {} result = {}, {:?}", opcode, instr, modes);
        Instruction { opcode: instr, modes: modes }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_example() {
        let mut p = Program::parse("1002,4,3,4,33").expect("Failed to parse input");
        p.execute(0);
    }

    #[test]
    fn p1_solution() {
        let mut p = Program::parse(INPUT).expect("Failed to parse input");
//        p.execute(1);
    }

    #[test]
    fn p2_solution() {
        let mut p = Program::parse(INPUT).expect("Failed to parse input");
        p.execute(5);
    }

}
