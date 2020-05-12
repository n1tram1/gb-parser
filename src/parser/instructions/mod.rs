pub mod operand;
pub mod nop;
pub mod add;
pub mod ld;

pub use super::InstructionTrait;
pub use operand::{Reg16, Operand};
pub use nop::Nop;
pub use add::Add;
pub use ld::Ld;

use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Nop(Nop),
    Ld(Ld),
}

impl Instruction {
    fn get_instruction(&self) -> &dyn InstructionTrait {
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
