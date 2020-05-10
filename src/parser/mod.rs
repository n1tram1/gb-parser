mod instructions;

mod test_parser;

pub trait InstructionTrait {
    fn size(&self) -> usize;
}

pub fn parse(bytes: &[u8]) -> Result<instructions::Instruction, &'static str> {
    if bytes.len() < 1 {
        return Err("Not enough bytes to parse an instruction");
    }

    let opcode = bytes[0];
    let bytes = &bytes[1..];
    decode(opcode, bytes)
}

fn decode(opcode: u8, bytes: &[u8]) -> Result<instructions::Instruction, &'static str> {
    let inst = match opcode {
        0x00 => instructions::Nop::new(),
        _ => return Err("invalid instruction"),
    };

    Ok(inst.into())
}
