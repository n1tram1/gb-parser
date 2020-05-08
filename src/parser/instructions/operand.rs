#[derive(Debug, PartialEq)]
pub enum Reg16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

#[derive(Debug, PartialEq)]
pub enum Operand {
    Imm8(u8),
    Reg16(Reg16),
}

impl Operand {
    pub fn encoding_size(&self) -> usize {
        match self {
            Self::Imm8(_) => 1,
            _ => 0,
        }
    }
}
