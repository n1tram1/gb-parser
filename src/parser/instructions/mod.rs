pub mod operand;
pub mod nop;
pub mod add;

pub use super::InstructionTrait;
pub use operand::{Reg16, Operand};
pub use nop::Nop;
pub use add::Add;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Nop(Nop),
    Add(Add),
}

impl Instruction {
    fn get_instruction(&self) -> &dyn InstructionTrait {
        match self {
            Self::Nop(inst) => inst,
            Self::Add(inst) => inst,
        }
    }
}
