mod instructions;
mod error;

mod test_parser;

pub trait InstructionTrait {
    fn size(&self) -> usize;
}

pub fn parse(bytes: &[u8]) -> Result<instructions::Instruction, error::ParserError> {
    if bytes.len() < 1 {
        return Err(error::ParserError::NotEnoughBytes(bytes.len()));
    }

    let opcode = bytes[0];
    let bytes = &bytes[1..];
    decode(opcode, bytes)
}

fn decode(opcode: u8, bytes: &[u8]) -> Result<instructions::Instruction, error::ParserError> {
    let inst: instructions::Instruction = match opcode {
        0x00 => instructions::Nop::new().into(),
        0x01 => instructions::Ld::new(instructions::Operand::Reg16(instructions::Reg16::BC), instructions::Operand::imm16_from_bytes(bytes)?, 3).into(),
        0x11 => instructions::Ld::new(instructions::Operand::Reg16(instructions::Reg16::DE), instructions::Operand::imm16_from_bytes(bytes)?, 3).into(),
        0x21 => instructions::Ld::new(instructions::Operand::Reg16(instructions::Reg16::HL), instructions::Operand::imm16_from_bytes(bytes)?, 3).into(),
        0x31 => instructions::Ld::new(instructions::Operand::Reg16(instructions::Reg16::SP), instructions::Operand::imm16_from_bytes(bytes)?, 3).into(),
        _ => return Err(error::ParserError::InvalidOpcode(opcode)),
    };

    Ok(inst)
}
