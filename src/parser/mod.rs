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
    let inst = match opcode {
        0x00 => instructions::Nop::new(),
        _ => return Err(error::ParserError::InvalidOpcode(opcode)),
    };

    Ok(inst.into())
}
