use std::num::ParseIntError;
use std::convert::TryFrom;

use super::instruction::*;

#[derive(Debug,Clone,Copy,PartialEq)]
pub enum ProgramState {
    Running(Option<isize>),
    Blocked,
    Halted
}

#[derive(Debug,Clone,PartialEq)]
pub struct Program {
    bytes: Vec<isize>,
    ip:    usize,
    base:  isize
}

impl Program {
    pub fn parse(input: &str) -> Result<Program, ParseIntError> {
        let p = input
            .trim()
            .split(",")
            .map(|part| part.trim().parse::<isize>())
            .collect::<Result<Vec<isize>, ParseIntError>>()?;
        Ok(Program { bytes: p, ip: 0, base: 0 })
    }

    pub fn step(&mut self, input: Option<isize>) -> Result<ProgramState, InvalidInstruction> {
        let instr = Instruction::try_from(self.bytes[self.ip] as usize)?;

        match instr.opcode {
            OpCode::Add => {
                let v3 = self.bytes[self.ip + 3];
                self.bytes[v3 as usize] = self.eval_binary(&instr, self.ip, |a, b| a + b);
                self.ip += 4;
            },
            OpCode::Multiply => {
                let v3 = self.bytes[self.ip + 3];
                self.bytes[v3 as usize] = self.eval_binary(&instr, self.ip, |a, b| a * b);
                self.ip += 4;
            },
            OpCode::Input => {
                if input.is_none() {
                    return Ok(ProgramState::Blocked);
                }
                let v1 = self.bytes[self.ip + 1];
                self.bytes[v1 as usize] = input.unwrap();
                self.ip += 2;
            },
            OpCode::Output => {
                let v1 = self.load_argument(self.bytes[self.ip + 1], instr.modes[0]);
                self.ip += 2;
                return Ok(ProgramState::Running(Some(v1)))
            },
            OpCode::JmpTrue => {
                let cp = self.ip;
                self.ip = self.eval_binary(&instr, self.ip, |a, b|{
                    if a != 0 { b as usize } else { cp + 3 }
                });
            },
            OpCode::JmpFalse => {
                let cp = self.ip;
                self.ip = self.eval_binary(&instr, self.ip, |a, b|{
                    if a == 0 { b as usize } else { cp + 3 }
                });
            },
            OpCode::LessThan => {
                let v3 = self.bytes[self.ip + 3];
                self.bytes[v3 as usize] = self.eval_binary(&instr, self.ip, |a, b|{
                    if a < b { 1 } else { 0 }
                });
                self.ip += 4;
            },
            OpCode::Equals => {
                let v3 = self.bytes[self.ip + 3];
                self.bytes[v3 as usize] = self.eval_binary(&instr, self.ip, |a, b|{
                    if a == b { 1 } else { 0 }
                });
                self.ip += 4;
            },
            OpCode::SetBase => {
                let v1 = self.load_argument(self.bytes[self.ip + 1], instr.modes[0]);
                self.base = v1;
                self.ip += 2;
            },
            OpCode::Halt => return Ok(ProgramState::Halted)
        };

        Ok(ProgramState::Running(None))
    }

    pub fn execute(&mut self, input: &Vec<isize>) -> Result<Vec<isize>, InvalidInstruction> {
        let mut results = vec![];
        let mut inp = input.iter();
        let mut next_input : Option<isize> = None;
        loop {
            let rc = self.step(next_input)?;
            next_input = None;

            match rc {
                ProgramState::Halted => break,
                ProgramState::Running(Some(v)) => results.push(v),
                ProgramState::Blocked => {
                    next_input = Some(*inp.next().expect("Ran out of input!!"));
                }
                _ => {}
            };
        }

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
            AddressMode::Immediate => value,
            AddressMode::Relative  => self.bytes[(self.base + value) as usize]
        }
    }
}

