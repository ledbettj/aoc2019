use std::num::ParseIntError;
use std::convert::TryFrom;
use std::collections::HashMap;

use super::instruction::*;

#[derive(Debug,Clone,Copy,PartialEq)]
pub enum ProgramState {
    Running(Option<isize>),
    Blocked,
    Halted
}

#[derive(Debug,Clone,PartialEq)]
pub struct Program {
    bytes: HashMap<usize, isize>,
    ip:    usize,
    base:  isize
}

impl Program {
    pub fn parse(input: &str) -> Result<Program, ParseIntError> {
        let p = input
            .trim()
            .split(",")
            .enumerate()
            .map(|(index, part)|{
                match part.trim().parse::<isize>() {
                    Ok(n)  => Ok((index, n)),
                    Err(e) => Err(e)
                }
            })
            .collect::<Result<HashMap<usize, isize>, ParseIntError>>()?;
        Ok(Program { bytes: p, ip: 0, base: 0 })
    }

    fn get_mem(&self, index: usize) -> isize {
        let v = *self.bytes.get(&index).unwrap_or(&0);
        v
    }

    fn set_mem(&mut self, index: usize, value: isize) {
        self.bytes.insert(index, value);
    }

    pub fn step(&mut self, input: Option<isize>) -> Result<ProgramState, InvalidInstruction> {
        let instr = Instruction::try_from(self.get_mem(self.ip) as usize)?;

        match instr.opcode {
            OpCode::Add => {
                let v3 = self.load_address(self.get_mem(self.ip + 3), instr.modes[2]);
                let new = self.eval_binary(&instr, self.ip, |a, b| a + b);
                self.set_mem(v3 as usize, new);
                self.ip += 4;
            },
            OpCode::Multiply => {
                let v3 = self.load_address(self.get_mem(self.ip + 3), instr.modes[2]);
                let new = self.eval_binary(&instr, self.ip, |a, b| a * b);
                self.set_mem(v3 as usize, new);
                self.ip += 4;
            },
            OpCode::Input => {
                if input.is_none() {
                    return Ok(ProgramState::Blocked);
                }
                let v1 = self.load_address(self.get_mem(self.ip + 1), instr.modes[0]);
                self.set_mem(v1 as usize, input.unwrap());
                self.ip += 2;
            },
            OpCode::Output => {
                let v1 = self.load_argument(self.get_mem(self.ip + 1),
                                            instr.modes[0]);
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
                let v3 = self.load_address(self.get_mem(self.ip + 3), instr.modes[2]);
                let new = self.eval_binary(&instr, self.ip, |a, b|{
                    if a < b { 1 } else { 0 }
                });
                self.set_mem(v3 as usize, new);
                self.ip += 4;
            },
            OpCode::Equals => {
                let v3 = self.load_address(self.get_mem(self.ip + 3), instr.modes[2]);
                let new = self.eval_binary(&instr, self.ip, |a, b|{
                    if a == b { 1 } else { 0 }
                });
                self.set_mem(v3 as usize, new);
                self.ip += 4;
            },
            OpCode::SetBase => {
                let v1 = self.load_argument(self.get_mem(self.ip + 1), instr.modes[0]);
                self.base += v1;
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
        let v1 = self.load_argument(self.get_mem(index + 1), instr.modes[0]);
        let v2 = self.load_argument(self.get_mem(index + 2), instr.modes[1]);
        f(v1, v2)
    }

    pub fn load_argument(&self, value: isize, mode: AddressMode) -> isize {
        let v = match mode {
            AddressMode::Position  => self.get_mem(value as usize),
            AddressMode::Immediate => value,
            AddressMode::Relative  => self.get_mem((self.base + value) as usize)
        };
//        println!("load_argument: {}, {:?} => {}", value, mode, v);
        v
    }

    pub fn load_address(&self, value: isize, mode: AddressMode) -> usize {
        match mode {
            AddressMode::Position  => value as usize,
            AddressMode::Relative  => (self.base + value) as usize,
            AddressMode::Immediate => panic!("cannot get address of Immediate mode")
        }
    }
}

