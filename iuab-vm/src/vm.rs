use std::{convert::{TryFrom, TryInto}, io::prelude::*};
use iuab_utils::{error::Error, opcode::Opcode};
pub struct VM<R:Read, W:Write> {
    bytecode: Vec<u8>,
    memory: Vec<u8>,
    pc: usize,
    ptr: usize,
    reader: R,
    writer: W,
}

impl<R, W> VM<R, W> 
    where R: Read, W: Write {
    pub fn new(bytecode: Vec<u8>, reader: R, writer: W) -> Self {
        Self {
            bytecode,
            memory: vec![0; 1 << 16],
            pc: 0,
            ptr: 0,
            reader,
            writer
        }
    }
    pub fn run(&mut self) -> Result<(), Error> {
        loop {
            let token = self.read_byte();
            let opcode: Opcode = token.try_into()?;
            match opcode {
                Opcode::Ret => return Ok(()),
                Opcode::Incp | Opcode::Decp | Opcode::Incv | Opcode::Decv => {
                    let counter = self.read_byte();
                    if opcode == Opcode::Incp {
                        self.ptr += counter as usize;
                    } else if opcode == Opcode::Decp {
                        self.ptr -= counter as usize;
                    } else if opcode == Opcode::Incv {
                        self.memory[self.ptr] += counter;
                    } else {
                        self.memory[self.ptr] -= counter;
                    }
                }
                Opcode::Read => {
                    let mut buffer = [0u8];
                    self.reader.read_exact(&mut buffer)?;
                    self.memory[self.ptr] = buffer[0];
                }
                Opcode::Write => {
                    self.writer.write_fmt(format_args!("{}", self.memory[self.ptr] as char))?;
                }
                Opcode::Jmpz => {
                    let value = self.get_usize();
                    if self.memory[self.ptr] == 0 {
                        self.pc = value;
                    }
                }
                Opcode::Jmpnz => {
                    let value = self.get_usize();
                    if self.memory[self.ptr] != 0 {
                        self.pc = value;
                    }
                }
                Opcode::Debug => {
                    println!("debug: pc={:#02x} dp={:#02x} *dp={:#02x}\n", self.pc, self.ptr, self.memory[self.ptr]);
                }
            }
        }
    }
    pub fn read_byte(&mut self) -> u8 {
        self.pc += 1;
        return self.bytecode[self.pc - 1]
    }
    pub fn get_usize(&mut self) -> usize {
        let value = usize::from_ne_bytes(self.bytecode[self.pc..self.pc+std::mem::size_of::<usize>()].try_into().unwrap());
        self.pc += std::mem::size_of::<usize>();
        value
    }
}