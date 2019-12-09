use std::error::Error;
use std::convert::TryFrom;
use std::fmt;

use num_traits::cast::FromPrimitive;

#[derive(Debug,Clone,Copy,PartialEq,FromPrimitive)]
pub enum AddressMode {
    Position  = 0,
    Immediate = 1,
    Relative  = 2
}

#[derive(Debug,Clone,Copy,PartialEq,FromPrimitive)]
pub enum OpCode {
    Add      = 1,
    Multiply = 2,
    Input    = 3,
    Output   = 4,
    JmpTrue  = 5,
    JmpFalse = 6,
    LessThan = 7,
    Equals   = 8,
    SetBase  = 9,
    Halt     = 99
}

#[derive(Debug,PartialEq)]
pub struct Instruction {
    pub opcode: OpCode,
    pub modes:  Vec<AddressMode>
}

#[derive(Debug,PartialEq)]
pub enum InvalidInstruction {
    InvalidOpcode,
    InvalidAddressMode
}

// Required for Error trait
impl fmt::Display for InvalidInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl Error for InvalidInstruction {}

impl TryFrom<usize> for Instruction {
    type Error = InvalidInstruction;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let opcode = OpCode::from_usize(value % 100)
            .ok_or(InvalidInstruction::InvalidOpcode)?;

        let mode_values = vec![
            (value / 100)    % 10,
            (value / 1_000)  % 10,
            (value / 10_000) % 10,
        ];

        let modes = mode_values
            .iter()
            .map(|&v|{
                AddressMode::from_usize(v).ok_or(InvalidInstruction::InvalidAddressMode)
            })
            .collect::<Result<Vec<AddressMode>, InvalidInstruction>>()?;

        Ok(Instruction { opcode: opcode, modes:  modes })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_success() {
        assert_eq!(Instruction::try_from(1002), Ok(Instruction {
            opcode: OpCode::Multiply,
            modes: vec![
                AddressMode::Position,
                AddressMode::Immediate,
                AddressMode::Position
            ]
        }));
    }

    #[test]
    fn try_from_fail() {
        assert_eq!(Instruction::try_from(3002), Err(InvalidInstruction::InvalidAddressMode));
        assert_eq!(Instruction::try_from(10032), Err(InvalidInstruction::InvalidOpcode));
    }
}
