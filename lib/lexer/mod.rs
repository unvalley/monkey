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

    fn is_letter(&mut self, ch: u8) -> bool {
        (b'a'..=b'z').contains(&ch) || (b'A'..=b'Z').contains(&ch) || ch == b'_'
    }

    fn is_digit(&mut self, ch: u8) -> bool {
        (b'0'..=b'9').contains(&ch)
    }

    fn read_int(&mut self, ch: u8) -> i64 {
        let position = self.position;
        while self.is_digit(ch) {
            self.read_char()
        }
        let int = &self.input[position..self.position];
        int.parse::<i64>().unwrap()
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.is_letter(self.ch) {
            self.read_char()
        }
        self.input[position..self.position].to_string()
    }

    fn skip_whitespace(&mut self) {
        while let b' ' | b'\t' | b'\n' | b'\r' = self.ch {
            self.read_char()
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            b'=' => Token::Assign,
            b'+' => Token::Plus,
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b'{' => Token::LBrace,
            b'}' => Token::RBrace,
            _ => {
                if self.is_letter(self.ch) {
                    let literal = self.read_identifier();
                    match literal.as_str() {
                        "fn" => Token::Function,
                        "let" => Token::Let,
                        _ => Token::Ident(literal)
                    }
                } else if self.is_digit(self.ch) {
                    let int = self.read_int(self.ch);
                    Token::IntLiteral(int)
                } else {
                    Token::Illegal
                }
            },
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
        let input = "let five = 5;
        let ten = 10;

        let add = fn(x, y) {
            x + y;
        };
        let result = add(five, ten);
        ";
        let expected_tokens = vec![
            Token::Let,
            Token::Ident(String::from("five")),
            Token::Assign,
            Token::IntLiteral(5),
            Token::SemiColon,
            Token::Let,
            Token::Ident(String::from("ten")),
            Token::Assign,
            Token::IntLiteral(10),
            Token::SemiColon,
            Token::Let,
            Token::Ident(String::from("add")),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Ident(String::from("x")),
            Token::Comma,
            Token::Ident(String::from("y")),
            Token::RParen,
            Token::LBrace,
            Token::Ident(String::from("x")),
            Token::Plus,
            Token::Ident(String::from("y")),
            Token::SemiColon,
            Token::RBrace,
            Token::SemiColon,
            Token::Let,
            Token::Ident(String::from("result")),
            Token::Assign,
            Token::Ident(String::from("add")),
            Token::LBrace,
            Token::Ident(String::from("five")),
            Token::Comma,
            Token::Ident(String::from("ten")),
            Token::SemiColon,
        ];

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
