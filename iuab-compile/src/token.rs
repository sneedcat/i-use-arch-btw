#[allow(dead_code)]
#[derive(Debug, PartialEq)]
#[repr(u8)]
/// Tokens from lexer
pub enum Token {
    I = 0,
    Use = 1,
    Arch = 2,
    Linux = 3,
    Btw = 4,
    By = 5,
    The = 6,
    Way = 7,
    Gentoo = 8,
    Eof = 9,
    Undefined = 10,
}