use std::error;
use std::fmt;


#[derive(Debug, Clone)]
pub enum ParserError {
    InvalidOpcode(u8),
    NotEnoughBytes(usize),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::InvalidOpcode(op) =>
                write!(f, "invalid opcode: {:#X}", op),
            Self::NotEnoughBytes(len) =>
                write!(f, "not enough bytes to be parsed ({} bytes)", len),
        }
    }
}

impl error::Error for ParserError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Self::InvalidOpcode(_) => None,
            Self::NotEnoughBytes(_) => None,
        }
    }
}
