#[allow(dead_code)]
use std::fmt::{Debug, Display};

/// Error enum for compile and vm
#[allow(dead_code)]
#[derive(PartialEq, Eq)]
pub enum Error {
    // Compiler errors
    InvalidToken { row: usize, col: usize},
    DepthMin,
    DepthNz,
    // VM errors
    InvalidOp,
    PtrOverflow,
    PtrUnderflow,
    InputEof,
    // Parse error
    OpcodeParse(u8),
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidToken{row, col}=> write!(f, "INVALID_TOKEN at {} row and {} col", row, col),
            Self::DepthMin => write!(f, "DEPTH_MIN"),
            Self::DepthNz => write!(f, "DEPTH_NZ"),
            Self::InvalidOp => write!(f, "INVALID_OP"),
            Self::PtrOverflow => write!(f, "PTR_OVERFLOW"),
            Self::PtrUnderflow => write!(f, "PTR_UNDERFLOW"),
            Self::InputEof => write!(f, "INPUT_EOF"),
            Self::OpcodeParse(n) => write!(f, "Invalid Opcode parse {}", n),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Error::InputEof
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", self)
    }
}

impl std::error::Error for Error {}