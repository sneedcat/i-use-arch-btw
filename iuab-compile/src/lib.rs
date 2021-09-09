pub mod compile;
mod token;
mod lexer;

#[cfg(test)]
mod tests {
    use crate::compile;
    use iuab_utils::error::Error;
    // oficial test
    #[test]
    fn test1() {
        let code = include!("../tests/test1.archbtw");
        let mut compiler = compile::Compiler::new(code);
        let result = compiler.compile().unwrap();
        assert_eq!(result, &[3, 1, 7, 22, 0, 0, 0, 0, 0, 0, 0, 5, 6, 8, 11, 0, 0, 0, 0, 0, 0, 0, 0]);
    }
    // empty buffer
    #[test]
    fn test2() {
        let code = include!("../tests/test2.archbtw");
        let mut compiler = compile::Compiler::new(code);
        let result = compiler.compile().unwrap();
        assert_eq!(result, &[0]);
    }
    // invalid token
    #[test]
    fn test3() {
        let code = include!("../tests/test3.archbtw");
        let mut compiler = compile::Compiler::new(code());
        assert_eq!(compiler.compile().err().unwrap(), Error::InvalidToken{row: 0, col: 2});
    }
    // same as above
    #[test]
    fn test4() {
        let code = include!("../tests/test4.archbtw");
        let mut compiler = compile::Compiler::new(code());
        assert_eq!(compiler.compile().err().unwrap(), Error::DepthMin);
    }
}
