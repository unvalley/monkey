pub mod token;
use crate::lexer::token::*;

#[derive(Debug, Clone)]
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

    fn read_int(&mut self) -> i64 {
        let position = self.position;
        while self.is_digit(self.ch) {
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

    fn read_string(&mut self) -> Token {
        let position = self.position + 1;
        loop {
            self.read_char();
            if self.ch == b'"' || self.ch == 0 {
                break
            }
        }
        let string = self.input[position..self.position].to_string();
        Token::StringLiteral(string)
    }

    fn skip_whitespace(&mut self) {
        while let b' ' | b'\t' | b'\n' | b'\r' = self.ch {
            self.read_char()
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            b'=' => {
                if let b'=' = self.peek_char() {
                    self.read_char();
                    Token::Eq
                } else {
                    Token::Assign
                }
            }
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b'!' => {
                if let b'=' = self.peek_char() {
                    self.read_char();
                    Token::NotEq
                } else {
                    Token::Bang
                }
            }
            b'/' => Token::Slash,
            b'*' => Token::Asterisk,
            b'<' => Token::LT,
            b'>' => Token::GT,
            b';' => Token::SemiColon,
            b',' => Token::Comma,
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b'{' => Token::LBrace,
            b'}' => Token::RBrace,
            b'"' => self.read_string(),
            0 => Token::EOF,
            _ => {
                if self.is_letter(self.ch) {
                    let literal = self.read_identifier();
                    return match literal.as_str() {
                        "fn" => Token::Function,
                        "let" => Token::Let,
                        "if" => Token::If,
                        "else" => Token::Else,
                        "return" => Token::Return,
                        "true" => Token::True,
                        "false" => Token::False,
                        _ => Token::Identifier(literal),
                    };
                } else if self.is_digit(self.ch) {
                    let int = self.read_int();
                    return Token::IntLiteral(int);
                }
                Token::Illegal
            }
        };
        self.read_char();
        tok
    }

    /// want to peek only. pre-read
    fn peek_char(&mut self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input.as_bytes()[self.read_position]
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_token() {
        let input = r#"let five = 5;
        let ten = 10;

        let add = fn(x, y) {
            x + y;
        };
        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;
        "foobar"
        "foo bar"
        "#;
        let expected_tokens = vec![
            Token::Let,
            Token::Identifier(String::from("five")),
            Token::Assign,
            Token::IntLiteral(5),
            Token::SemiColon,
            Token::Let,
            Token::Identifier(String::from("ten")),
            Token::Assign,
            Token::IntLiteral(10),
            Token::SemiColon,
            Token::Let,
            Token::Identifier(String::from("add")),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Identifier(String::from("x")),
            Token::Comma,
            Token::Identifier(String::from("y")),
            Token::RParen,
            Token::LBrace,
            Token::Identifier(String::from("x")),
            Token::Plus,
            Token::Identifier(String::from("y")),
            Token::SemiColon,
            Token::RBrace,
            Token::SemiColon,
            Token::Let,
            Token::Identifier(String::from("result")),
            Token::Assign,
            Token::Identifier(String::from("add")),
            Token::LParen,
            Token::Identifier(String::from("five")),
            Token::Comma,
            Token::Identifier(String::from("ten")),
            Token::RParen,
            Token::SemiColon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::IntLiteral(5),
            Token::SemiColon,
            Token::IntLiteral(5),
            Token::LT,
            Token::IntLiteral(10),
            Token::GT,
            Token::IntLiteral(5),
            Token::SemiColon,
            Token::StringLiteral(String::from("foobar")),
            Token::StringLiteral(String::from("foo bar")),
            Token::EOF,
        ];

        let mut l = Lexer::new(input.to_string());

        for expected in expected_tokens {
            let actual = l.next_token();
            assert_eq!(
                expected, actual,
                "tests - token type wrong. expected={:?}, actual={:?}",
                expected, actual,
            )
        }
    }
}
