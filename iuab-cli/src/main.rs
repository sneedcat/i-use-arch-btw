use iuab_compile::compile::Compiler;
use iuab_vm::vm::VM;
use structopt::StructOpt;
use std::{fs::File, io::{BufWriter, prelude::*}, time::Instant};
mod opts;

fn main() {
    let opts = opts::Opt::from_args();
    let start = Instant::now();
    let mut code = Vec::new();
    let mut file = File::open(opts.input).unwrap();
    file.read_to_end(&mut code).unwrap();
    let mut compiler = Compiler::new(code);
    let bytecode= compiler.compile().unwrap();
    let writer: Box<dyn Write> = match opts.output {
        Some(output) => Box::new(BufWriter::new(File::create(output).unwrap())),
        None => Box::new(std::io::stdout()),
    };
    let mut vm = VM::new(bytecode, std::io::stdin(), writer);
    vm.run().unwrap();
    if opts.debug {
        let duration = start.elapsed();
        println!("Time elapsed in execution: {:?}", duration);
    }
}