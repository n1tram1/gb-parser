use super::Instruction;
use super::InstructionTrait;
use super::operand::Operand;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Xor {
    operand: Operand,
}

impl Xor {
    pub fn new(operand: Operand) -> Xor {
        Xor { operand }
    }

    pub fn get_operand(&self) -> &Operand {
        &self.operand
    }
}

impl From<Xor> for Instruction {
    fn from(item: Xor) -> Self {
        Self::Xor(item)
    }
}

impl std::fmt::Display for Xor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "XOR {}", self.operand)
    }
}

impl InstructionTrait for Xor {
    fn size(&self) -> usize {
        if let Operand::Imm8(_) = self.operand {
            2
        } else {
            1
        }
    }
}
