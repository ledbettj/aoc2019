use std::num::ParseIntError;
use std::convert::TryFrom;

use super::instruction::*;

#[derive(Debug,Clone,PartialEq)]
pub struct Program {
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

    pub fn execute(&mut self, input: &Vec<isize>) -> Result<Vec<isize>, InvalidInstruction> {
        let mut i = 0;
        let mut results = vec![];
        let mut inp = input.iter();

        while i < self.bytes.len() {
            let instr = Instruction::try_from(self.bytes[i] as usize)?;

            match instr.opcode {
                OpCode::Add => {
                    let v3 = self.bytes[i + 3];
                    self.bytes[v3 as usize] = self.eval_binary(&instr, i as usize, |a, b| a + b);
                    i += 4;
                },
                OpCode::Multiply => {
                    let v3 = self.bytes[i + 3];
                    self.bytes[v3 as usize] = self.eval_binary(&instr, i as usize, |a, b| a * b);
                    i += 4;
                },
                OpCode::Input => {
                    let v1 = self.bytes[i + 1];
                    self.bytes[v1 as usize] = *inp.next().expect("Prompted for input but no more left!");
                    i += 2;
                },
                OpCode::Output => {
                    let v1 = self.load_argument(self.bytes[i + 1], instr.modes[0]);
                    results.push(v1);
                    i += 2;
                },
                OpCode::JmpTrue => {
                    i = self.eval_binary(&instr, i as usize, |a, b|{
                        if a != 0 { b as usize } else { i + 3 }
                    });
                },
                OpCode::JmpFalse => {
                    i = self.eval_binary(&instr, i as usize, |a, b|{
                        if a == 0 { b as usize } else { i + 3 }
                    });
                },
                OpCode::LessThan => {
                    let v3 = self.bytes[i + 3];
                    self.bytes[v3 as usize] = self.eval_binary(&instr, i as usize, |a, b|{
                        if a < b { 1 } else { 0 }
                    });
                    i += 4;
                },
                OpCode::Equals => {
                    let v3 = self.bytes[i + 3];
                    self.bytes[v3 as usize] = self.eval_binary(&instr, i as usize, |a, b|{
                        if a == b { 1 } else { 0 }
                    });
                    i += 4;
                }
                OpCode::Halt => break
            };
        };

        Ok(results)
    }

    fn eval_binary<F, T>(&mut self, instr: &Instruction, index: usize, f: F) -> T
        where F : Fn(isize, isize) -> T
    {
        let v1 = self.load_argument(self.bytes[index + 1], instr.modes[0]);
        let v2 = self.load_argument(self.bytes[index + 2], instr.modes[1]);
        f(v1, v2)
    }

    pub fn load_argument(&self, value: isize, mode: AddressMode) -> isize {
        match mode {
            AddressMode::Position  => self.bytes[value as usize],
            AddressMode::Immediate => value
        }
    }
}

