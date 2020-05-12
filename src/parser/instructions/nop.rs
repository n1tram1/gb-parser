use super::Instruction;
use super::InstructionTrait;

#[derive(Debug, PartialEq)]
pub struct Nop {}

impl Nop {
    pub fn new() -> Nop {
        Nop {}
    }

    pub fn new_instruction() -> Instruction {
        Instruction::Nop(Self::new())
    }
}

impl From<Nop> for Instruction {
    fn from(item: Nop) -> Self {
        Self::Nop(item)
    }
}

impl std::fmt::Display for Nop {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "NOP")
    }
}

impl InstructionTrait for Nop {
    fn size(&self) -> usize {
        1
    }
}
