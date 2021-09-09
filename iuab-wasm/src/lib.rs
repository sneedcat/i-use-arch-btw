use iuab_compile::compile::Compiler;
use iuab_utils::error::Error;
use iuab_vm::vm::VM;
use wasm_bindgen::prelude::*;

mod utils;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(catch)]
pub fn iuab(code: &[u8], reader: &[u8]) -> Vec<u8> {
    utils::set_panic_hook();
    let mut compiler = Compiler::new(code.to_owned());
    let bytecode = match compiler.compile() {
        Ok(v) => v,
        Err(err) => panic!("{:?}", err),
    };
    let mut writer = Vec::new();
    let mut vm = VM::new(bytecode, reader, &mut writer);
    match vm.run() {
        Ok(_) => (),
        Err(err) => panic!("{:?}", err),
    }
    writer
}
