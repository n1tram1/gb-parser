pub mod operand;
pub mod nop;
pub mod ld;

use super::error;
use super::InstructionTrait;
use operand::{Reg16, Operand};
use nop::Nop;
use ld::Ld;

use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Nop(Nop),
    Ld(Ld),
}

impl Instruction {
    pub fn get_instruction(&self) -> &dyn InstructionTrait {
        match self {
            Self::Nop(inst) => inst,
            Self::Ld(inst) => inst,
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Nop(nop) => write!(f, "{}", nop),
            Self::Ld(ld) => write!(f, "{}", ld),
        }
    }
}

pub fn decode(opcode: u8, bytes: &[u8]) -> Result<Instruction, error::ParserError> {
    let inst: Instruction = match opcode {
        0x00 => Nop::new().into(),
        0x01 => Ld::new(Operand::Reg16(Reg16::BC), Operand::imm16_from_bytes(bytes)?, 3).into(),
        0x11 => Ld::new(Operand::Reg16(Reg16::DE), Operand::imm16_from_bytes(bytes)?, 3).into(),
        0x21 => Ld::new(Operand::Reg16(Reg16::HL), Operand::imm16_from_bytes(bytes)?, 3).into(),
        0x31 => Ld::new(Operand::Reg16(Reg16::SP), Operand::imm16_from_bytes(bytes)?, 3).into(),
        _ => return Err(error::ParserError::InvalidOpcode(opcode)),
    };

    Ok(inst)
}
