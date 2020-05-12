pub use super::InstructionTrait;
pub use super::Instruction;
pub use super::operand::Operand;

#[derive(Debug, PartialEq)]
pub struct Add {
    left_addend: Operand,
    right_addend: Operand,
}

impl Add {
    pub fn new(left_addend: Operand, right_addend: Operand) -> Add {
        Add { left_addend, right_addend }
    }
}

// impl From<Add> for Instruction {
//     fn from(item: Add) -> Self {
//         Self::Add(item)
//     }
// }

impl InstructionTrait for Add {
    fn size(&self) -> usize {
        1 + self.left_addend.encoding_size() + self.right_addend.encoding_size()
    }
}
