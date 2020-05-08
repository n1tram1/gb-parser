use super::Instruction;
use super::InstructionTrait;

#[derive(Debug, PartialEq)]
pub struct Nop {}

impl Nop {
    pub fn new() -> Nop {
        Nop {}
    }
}

impl From<Nop> for Instruction {
    fn from(item: Nop) -> Self {
        Self::Nop(item)
    }
}

impl InstructionTrait for Nop {
    fn size(&self) -> usize {
        1
    }
}
