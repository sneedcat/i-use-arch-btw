use std::convert::TryFrom;

use crate::error::Error;

#[allow(dead_code)]
#[derive(PartialEq, Clone, Debug)]
#[repr(u8)]
/// Opcodes used in representation
pub enum Opcode {
    Ret = 0,
    Incp = 1,
    Decp = 2,
    Incv = 3,
    Decv = 4,
    Read = 5,
    Write = 6,
    Jmpz = 7,
    Jmpnz = 8,
    Debug = 9,
}


// Try from
impl TryFrom<u8> for Opcode {
    type Error = crate::error::Error;
    fn try_from(n: u8) -> Result<Self, Error> {
        match n {
            0 => Ok(Self::Ret),
            1 => Ok(Self::Incp),
            2 => Ok(Self::Decp),
            3 => Ok(Self::Incv),
            4 => Ok(Self::Decv),
            5 => Ok(Self::Read),
            6 => Ok(Self::Write),
            7 => Ok(Self::Jmpz),
            8 => Ok(Self::Jmpnz),
            9 => Ok(Self::Debug),
            _ => Err(Error::OpcodeParse(n)),
        }
    }
}