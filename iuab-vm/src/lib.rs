pub mod vm;

#[cfg(test)]
mod tests {
    use crate::vm::VM;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn test_vm() {
        let reader = vec![0; 1000];
        let mut writer = Vec::new();
        let mut vm = VM::new(
            [3, 1, 7, 22, 0, 0, 0, 0, 0, 0, 0, 5, 6, 8, 11, 0, 0, 0, 0, 0, 0, 0, 0].into(), 
            reader.as_slice(), 
            &mut writer
        );
        vm.run().unwrap();
        println!("{:?}", &writer[..]);
    }
}
