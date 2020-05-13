pub mod operand;
pub mod nop;
pub mod ld;
pub mod xor;

use super::error;
use super::InstructionTrait;
use operand::{Reg8, Reg16, Operand};
use nop::Nop;
use ld::Ld;
use xor::Xor;

use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Nop(Nop),
    Ld(Ld),
    Xor(Xor),
}

impl Instruction {
    pub fn get_instruction(&self) -> &dyn InstructionTrait {
        match self {
            Self::Nop(inst) => inst,
            Self::Ld(inst) => inst,
            Self::Xor(inst) => inst,
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Nop(nop) => write!(f, "{}", nop),
            Self::Ld(ld) => write!(f, "{}", ld),
            Self::Xor(xor) => write!(f, "{}", xor),
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
        0xA8 => Xor::new(Operand::Reg8(Reg8::B)).into(),
        0xA9 => Xor::new(Operand::Reg8(Reg8::C)).into(),
        0xAA => Xor::new(Operand::Reg8(Reg8::D)).into(),
        0xAB => Xor::new(Operand::Reg8(Reg8::E)).into(),
        0xAC => Xor::new(Operand::Reg8(Reg8::H)).into(),
        0xAD => Xor::new(Operand::Reg8(Reg8::L)).into(),
        0xAE => Xor::new(Operand::DerefReg(Reg16::HL)).into(),
        0xAF => Xor::new(Operand::Reg8(Reg8::A)).into(),
        0xEE => Xor::new(Operand::imm8_from_bytes(bytes)?).into(),
        _ => return Err(error::ParserError::InvalidOpcode(opcode)),
    };

    Ok(inst)
}
