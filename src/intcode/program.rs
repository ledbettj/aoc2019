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
    mem:  HashMap<usize, isize>,
    ip:   usize,
    base: isize
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

        Ok(Program { mem: p, ip: 0, base: 0 })
    }

    fn get_mem(&self, index: usize) -> isize {
        *self.mem.get(&index).unwrap_or(&0)
    }

    pub fn set_mem(&mut self, index: usize, value: isize) {
        self.mem.insert(index, value);
    }

    pub fn step(&mut self, input: Option<isize>) -> Result<ProgramState, InvalidInstruction> {
        let instr = Instruction::try_from(self.get_mem(self.ip) as usize)?;

        match instr.opcode {
            OpCode::Add      => self.eval_basic(&instr, |a, b| a + b)?,
            OpCode::Multiply => self.eval_basic(&instr, |a, b| a * b)?,
            OpCode::Input    => if !self.eval_input(&instr, &input)? { return Ok(ProgramState::Blocked); },
            OpCode::Output   => return Ok(ProgramState::Running(Some(self.eval_output(&instr)))),
            OpCode::JmpTrue  => self.eval_jmp(&instr, |a, b| if a != 0 { Some(b) } else { None }),
            OpCode::JmpFalse => self.eval_jmp(&instr, |a, b| if a == 0 { Some(b) } else { None }),
            OpCode::LessThan => self.eval_basic(&instr, |a, b| if a < b { 1 } else { 0 })?,
            OpCode::Equals   => self.eval_basic(&instr, |a, b| if a == b { 1 } else { 0 })?,
            OpCode::SetBase  => self.eval_set_base(&instr),
            OpCode::Halt     => return Ok(ProgramState::Halted)
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

    fn eval_basic<F>(&mut self, instr: &Instruction, f: F) -> Result<(), InvalidInstruction>
    where F : Fn(isize, isize) -> isize {
        let arg1 = self.load_argument(self.get_mem(self.ip + 1), instr.modes[0]);
        let arg2 = self.load_argument(self.get_mem(self.ip + 2), instr.modes[1]);
        let addr = self.load_address(self.get_mem(self.ip + 3),  instr.modes[2])? as usize;

        let value = f(arg1, arg2);
        self.set_mem(addr, value);
        self.ip += 4;

        Ok(())
    }

    fn eval_jmp<F>(&mut self, instr: &Instruction, f: F)
    where F : Fn(isize, isize) -> Option<isize> {
        let arg1 = self.load_argument(self.get_mem(self.ip + 1), instr.modes[0]);
        let arg2 = self.load_argument(self.get_mem(self.ip + 2), instr.modes[1]);

        self.ip = match f(arg1, arg2) {
            Some(v) => v as usize,
            None    => self.ip + 3
        };
    }

    fn eval_set_base(&mut self, instr: &Instruction) {
        let arg = self.load_argument(self.get_mem(self.ip + 1), instr.modes[0]);
        self.base += arg;
        self.ip += 2;
    }

    fn eval_input(&mut self, instr: &Instruction, input: &Option<isize>) -> Result<bool, InvalidInstruction> {
        if input.is_none() {
            return Ok(false);
        }

        let arg = self.load_address(self.get_mem(self.ip + 1), instr.modes[0])?;
        self.set_mem(arg as usize, input.unwrap());
        self.ip += 2;

        Ok(true)
    }

    fn eval_output(&mut self, instr: &Instruction) -> isize {
        let arg = self.load_argument(self.get_mem(self.ip + 1), instr.modes[0]);
        self.ip += 2;
        return arg;
    }

    fn load_argument(&self, value: isize, mode: AddressMode) -> isize {
        match mode {
            AddressMode::Position  => self.get_mem(value as usize),
            AddressMode::Immediate => value,
            AddressMode::Relative  => self.get_mem((self.base + value) as usize)
        }
    }

    fn load_address(&self, value: isize, mode: AddressMode) -> Result<usize, InvalidInstruction> {
        match mode {
            AddressMode::Position  => Ok(value as usize),
            AddressMode::Relative  => Ok((self.base + value) as usize),
            AddressMode::Immediate => Err(InvalidInstruction::AttemptedImmediateLoad)
        }
    }
}

