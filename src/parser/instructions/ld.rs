use super::Instruction;
use super::InstructionTrait;
pub use super::operand::Operand;

#[derive(Debug, PartialEq)]
pub struct Ld {
    dst: Operand,
    src: Operand,
    size: usize,
}

impl Ld {
    pub fn new(dst: Operand, src: Operand, size: usize) -> Ld {
        Ld { dst, src, size }
    }

    pub fn get_dst(&self) -> &Operand {
        &self.dst
    }

    pub fn get_src(&self) -> &Operand {
        &self.src
    }
}

impl From<Ld> for Instruction {
    fn from(item: Ld) -> Self {
        Self::Ld(item)
    }
}

impl std::fmt::Display for Ld {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "LD {}, {}", self.dst, self.src)
    }
}

impl InstructionTrait for Ld {
    fn size(&self) -> usize {
        self.size
    }
}
