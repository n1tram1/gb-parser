use super::super::error;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Reg16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

impl std::fmt::Display for Reg16 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Operand {
    Imm8(u8),
    Imm16(u16),
    Reg16(Reg16),
}

impl Operand {
    pub fn imm8_from_bytes(bytes: &[u8]) -> Result<Operand, error::ParserError> {
        if bytes.len() < 1 {
            return Err(error::ParserError::NotEnoughBytes(bytes.len()));
        }

        Ok(Operand::Imm8(u8::from_le_bytes([bytes[0]])))
    }

    pub fn imm16_from_bytes(bytes: &[u8]) -> Result<Operand, error::ParserError> {
        if bytes.len() < 2 {
            return Err(error::ParserError::NotEnoughBytes(bytes.len()));
        }

        Ok(Operand::Imm16(u16::from_le_bytes([bytes[0], bytes[1]])))
    }

    pub fn is_immediate(&self) -> bool {
        match self {
            Self::Imm8(_) | Self::Imm16(_) => true,
            _ => false,
        }
    }

    pub fn is_register(&self) -> bool {
        match self {
            Self::Reg16(_) => true,
            _ => false,
        }
    }
}

impl Operand {
    pub fn encoding_size(&self) -> usize {
        match self {
            Self::Imm8(_) => 1,
            _ => 0,
        }
    }
}

impl std::fmt::Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Imm8(byte) => write!(f, "{:#X}", byte),
            Self::Imm16(word) => write!(f, "{:#X}", word),
            Self::Reg16(reg16) => write!(f, "{}", reg16),
        }
    }
}
