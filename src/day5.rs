use std::num::ParseIntError;
use num_traits::cast::FromPrimitive;

const INPUT : &'static str  = include_str!("../inputs/day5.txt");

#[derive(Debug)]
struct Instruction {
    opcode: OpCode,
    modes:  Vec<AddressMode>
}

#[derive(Debug,Clone,Copy,FromPrimitive)]
enum AddressMode {
    Position  = 0,
    Immediate = 1
}

#[derive(Debug,Clone,Copy,FromPrimitive)]
enum OpCode {
    Add      = 1,
    Multiply = 2,
    Input    = 3,
    Output   = 4,
    JmpTrue  = 5,
    JmpFalse = 6,
    LessThan = 7,
    Equals   = 8,
    Halt     = 99
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

    pub fn execute(&mut self, input: isize) -> Vec<isize> {
        let mut i = 0;
        let mut results = vec![];

        while i < self.bytes.len() {
            let instr = self.parse_instruction(self.bytes[i] as usize);
            match instr.opcode {
                OpCode::Add => {
                    let v1 = self.load_argument(self.bytes[i + 1], instr.modes[0]);
                    let v2 = self.load_argument(self.bytes[i + 2], instr.modes[1]);
                    let v3 = self.bytes[i + 3];
                    self.bytes[v3 as usize] = v1 + v2;
                    i += 4;
                },
                OpCode::Multiply => {
                    let v1 = self.load_argument(self.bytes[i + 1], instr.modes[0]);
                    let v2 = self.load_argument(self.bytes[i + 2], instr.modes[1]);
                    let v3 = self.bytes[i + 3];
                    self.bytes[v3 as usize] = v1 * v2;
                    i += 4;
                },
                OpCode::Input => {
                    let v1 = self.bytes[i + 1];
                    self.bytes[v1 as usize] = input;
                    i += 2;
                },
                OpCode::Output => {
                    let v1 = self.load_argument(self.bytes[i + 1], instr.modes[0]);
                    results.push(v1);
                    i += 2;
                },
                OpCode::JmpTrue => {
                    let v1 = self.load_argument(self.bytes[i + 1], instr.modes[0]);
                    let v2 = self.load_argument(self.bytes[i + 2], instr.modes[1]);
                    if v1 != 0 {
                        i = v2 as usize;
                    } else {
                        i += 3;
                    }
                },
                OpCode::JmpFalse => {
                    let v1 = self.load_argument(self.bytes[i + 1], instr.modes[0]);
                    let v2 = self.load_argument(self.bytes[i + 2], instr.modes[1]);
                    if v1 == 0 {
                        i = v2 as usize;
                    } else {
                        i += 3;
                    }
                },
                OpCode::LessThan => {
                    let v1 = self.load_argument(self.bytes[i + 1], instr.modes[0]);
                    let v2 = self.load_argument(self.bytes[i + 2], instr.modes[1]);
                    let v3 = self.bytes[i + 3];
                    self.bytes[v3 as usize] = if v1 < v2 { 1 } else { 0 };
                    i += 4;
                },
                OpCode::Equals => {
                    let v1 = self.load_argument(self.bytes[i + 1], instr.modes[0]);
                    let v2 = self.load_argument(self.bytes[i + 2], instr.modes[1]);
                    let v3 = self.bytes[i + 3];
                    self.bytes[v3 as usize] = if v1 == v2 { 1 } else { 0 };
                    i += 4;
                }
                OpCode::Halt => break
            };
        };

        results
    }

    pub fn load_argument(&self, value: isize, mode: AddressMode) -> isize {
        match mode {
            AddressMode::Position  => self.bytes[value as usize],
            AddressMode::Immediate => value
        }
    }

    pub fn parse_instruction(&self, instr: usize) -> Instruction  {
        let opcode = OpCode::from_usize(instr % 100).expect("Failed to parse OpCode");

        let modes = vec![
            (instr / 100)    % 10,
            (instr / 1_000)  % 10,
            (instr / 10_000) % 10,
        ].iter()
            .map(|&v| AddressMode::from_usize(v).expect("Failed to parse AddressMode") )
            .collect();

        Instruction {
            opcode: opcode,
            modes:  modes
        }
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
        let output = p.execute(1);
        assert_eq!(output.last(), Some(&5074395_isize));
    }

    #[test]
    fn p2_solution() {
        let mut p = Program::parse(INPUT).expect("Failed to parse input");
        let output = p.execute(5);
        assert_eq!(output.last(), Some(&8346937_isize));
    }

}
