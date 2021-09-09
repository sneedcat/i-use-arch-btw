use crate::token::Token;
#[allow(dead_code)]
pub struct Lexer {
    buffer: Vec<u8>,
    pub row: usize,
    pub col: usize,
    pos: usize,
}
#[allow(dead_code)]
impl Lexer {
    /// creates a new lexer
    pub fn new(code: Vec<u8>) -> Lexer {
        Lexer { buffer: code, row: 0, col: 0, pos: 0 }
    }
    /// peek the next token in the buffer
    pub fn peek_token(&mut self) -> Token {
        let initial_pos = self.pos;
        let initial_row = self.row;
        let initial_col = self.col;
        let token = self.get_next_token();
        self.row = initial_row;
        self.col = initial_col;
        self.pos = initial_pos;
        token
    }
    /// gets the next token in the buffer
    pub fn get_token(&mut self) -> Token {
        self.get_next_token()
    }
    
    fn get_next_token(&mut self) -> Token {
        // loop until we hit a non whitespace or comment
        loop {
            // clear all whitespaces from the buffer
            self.remove_whitespaces();
            if self.pos >= self.buffer.len() {
                return Token::Eof;
            }
            let mut ok = false;
            match self.buffer[self.pos] as char {
                ';' => self.end_comment(),
                '\n' => self.newline(),
                _ => {ok = true;},
            };
            if ok {
                break;
            }
        }
        let ch = self.buffer[self.pos] as char;
        // try to get a token out of a string
        let token = match ch {
            'i' => self.check_token("i", Token::I),
            'u' => self.check_token("use", Token::Use),
            'a' => self.check_token("arch", Token::Arch),
            'l' => self.check_token("linux", Token::Linux),
            'b' => {
                if self.pos + 1 >= self.buffer.len() {
                    return Token::Undefined;
                }
                let next_ch = self.buffer[self.pos + 1];
                let token = match next_ch as char {
                    't' => self.check_token("btw", Token::Btw),
                    'y' => self.check_token("by", Token::By),
                    _ => Token::Undefined,
                };
                return token;
            }
            't' => self.check_token("the", Token::The),
            'w' => self.check_token("way", Token::Way),
            'g' => self.check_token("gentoo", Token::Gentoo),
            _ => Token::Undefined,
        };
        token
    }

    fn end_comment(&mut self) {
        while self.pos < self.buffer.len() && self.buffer[self.pos] != '\n' as u8 {
            self.pos += 1;
        }
        self.newline();
    }
    
    fn remove_whitespaces(&mut self) {
        while self.pos < self.buffer.len() && self.buffer[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
    }
    
    fn newline(&mut self) {
        self.row += 1;
        self.col = 0;
        self.pos += 1;
    }
    
    fn check_token(&mut self, word: &'static str, token: Token) -> Token {
        if self.finish_word(word) {
            token
        } else {
            Token::Undefined
        }
    }
    // check if the given word is the same as a keyword
    fn finish_word(&mut self, s: &'static str) -> bool {
        let start = self.pos;
        while self.pos < self.buffer.len() && !self.buffer[self.pos].is_ascii_whitespace() && self.pos - start < s.len() {
            if self.buffer[self.pos] != s.as_bytes()[self.pos - start] {
                return false;
            }
            self.pos += 1;
            self.col += 1;
        }
        if self.pos - start < s.len() {
            return false;
        }
        if self.pos < self.buffer.len() &&  !self.buffer[self.pos].is_ascii_whitespace() {
            return false;
        }
        return true;
    }
}