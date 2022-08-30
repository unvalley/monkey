pub mod token;
use crate::lexer::token::*;

#[derive(Debug)]
pub struct Lexer {
    input: String,
    /// current
    position: usize,
    /// next
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        let is_eof = self.read_position >= self.input.len();
        if is_eof {
            self.ch = 0
        } else {
            self.ch = self.input.as_bytes()[self.read_position]
        };
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let tok = match self.ch {
            b'=' => Token::Assign,
            b'+' => Token::Plus,
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b'{' => Token::LBrace,
            b'}' => Token::RBrace,
            _ => Token::Illegal,
        };
        self.read_char();
        tok
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_token() {
        let input = "=+(){},;";
        let expected_tokens = vec![Token::Assign, Token::Plus, Token::LParen, Token::RParen];

        let mut l = Lexer::new(input.to_string());

        for expected in expected_tokens {
            let actual = l.next_token();
            assert_eq!(
                expected,
                actual,
                "tests - token type wrong. expected={:?}, actual={:?}",
                expected,
                actual,
            )
        }
    }
}
