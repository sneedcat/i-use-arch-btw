use std::collections::VecDeque;
use crate::{lexer::Lexer, token::Token};
use iuab_utils::error::Error;
use iuab_utils::opcode::Opcode;
#[allow(dead_code)]
pub struct Compiler {
    lexer: Lexer,
    stack: VecDeque<usize>,
}
#[allow(dead_code)]
impl Compiler {
    /// create a new compiler
    pub fn new(code: Vec<u8>) -> Self {
        Self {
            lexer: Lexer::new(code),
            stack: VecDeque::new(),
        }
    }
    /// compile the code into bytecode
    pub fn compile(&mut self) -> Result<Vec<u8>, Error> {
        let mut bytecode = Vec::new();
        loop {
            let token = self.lexer.get_token();
            match token {
                Token::I | Token::Use | Token::Arch | Token::Linux => {
                    // count how many times this token repeats
                    let mut counter: u8 = 1;
                    while self.lexer.peek_token() == token {
                        self.lexer.get_token();
                        counter += 1;
                    }
                    // transform a token into an opcode
                    let opcode = match token {
                        Token::I => Opcode::Incp,
                        Token::Use => Opcode::Decp,
                        Token::Arch => Opcode::Incv,
                        Token::Linux => Opcode::Decv,
                        _ => unreachable!(),
                    };
                    // push into the buffer
                    bytecode.push(opcode as u8);
                    bytecode.push(counter);
                }
                Token::The | Token::Way => {
                    if token == Token::The {
                        bytecode.push(Opcode::Jmpz as u8);
                        // push allocate the size of an usize in the buffer
                        for _ in 0..std::mem::size_of::<usize>() {
                            bytecode.push(0);
                        }
                        // push the length of the buffer now
                        self.stack.push_back(bytecode.len());
                    } else {
                        match self.stack.pop_back() {
                            Some(n) => {
                                bytecode.push(Opcode::Jmpnz as u8);
                                // unsafe for the lulz
                                unsafe {
                                    // transmute an usize into an array of u8 and the size of an usize to keep endianess
                                    let usize_as_bytes: [u8; std::mem::size_of::<usize>()] = std::mem::transmute(n);
                                    // push all bytes in the buffer
                                    for byte in usize_as_bytes {
                                        bytecode.push(byte);
                                    }
                                };
                                unsafe {
                                    // transmute an usize into an array of u8 and the size of an usize to keep endianess
                                    let usize_as_bytes: [u8; std::mem::size_of::<usize>()] = std::mem::transmute(bytecode.len());
                                    let mut i = n - std::mem::size_of::<usize>();
                                    // replace the allocated bytes above with usize
                                    // c would be cleaner, because it doesn't care about sizes
                                    for byte in usize_as_bytes {
                                        bytecode[i] = byte;
                                        i += 1;
                                    }
                                }
                            }
                            // if there is no `while start` on the stack, return the error
                            None => {
                                return Err(Error::DepthMin);
                            }
                        }
                    }
                }
                Token::Btw | Token::By | Token::Gentoo => {
                    let opcode =  if token == Token::Btw {
                        Opcode::Write
                    } else if token == Token::By {
                        Opcode::Read
                    } else {
                        Opcode::Debug
                    };
                    bytecode.push(opcode as u8);
                }
                // if the token is undefined, then return an error
                Token::Undefined => {
                    return Err(Error::InvalidToken { row: self.lexer.row, col: self.lexer.col })
                },
                // eof, just push the return opcode
                Token::Eof => {
                    bytecode.push(Opcode::Ret as u8);
                    break;
                }
            }
        }
        // if there is a loop in the stack, return an error
        if self.stack.len() > 0 {
            return Err(Error::DepthMin);
        }
        Ok(bytecode)
    }
}