use crate::lexing::token::TokenType;

pub struct Lexer {
    pub input: String,
    pub pos: usize,
    read_pos: usize,
    char: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Self {
            input,
            pos: 0,
            read_pos: 0,
            char: '\0',
        };
        lexer.read_char();

        lexer
    }

    pub fn next_token(&mut self) -> TokenType {
        skip_white_space(self);

        if self.char == '=' {
            match self.peek_char() {
                '=' => {
                    self.read_char();
                    self.read_char();
                    return TokenType::Eq;
                }
                _ => {
                    self.read_char();
                    return TokenType::Assign;
                }
            }
        }

        if self.char == '!' {
            match self.peek_char() {
                '=' => {
                    self.read_char();
                    self.read_char();
                    return TokenType::NotEq;
                }
                _ => {
                    self.read_char();
                    return TokenType::Bang;
                }
            }
        }

        if self.char.is_ascii_alphanumeric() || self.char == '_' {
            return read_dynamic_token(self);
        }

        let token = self.char.to_string().into();
        self.read_char();

        token
    }

    fn read_char(&mut self) {
        self.char = if self.input.len() <= self.read_pos {
            '\0'
        } else {
            self.input.chars().nth(self.read_pos).expect(
                format!(
                    "couldn't read next token as the lexer's read_pos {} is out of bounds.",
                    self.read_pos,
                )
                .as_str(),
            )
        };
        self.pos = self.read_pos;
        self.read_pos += 1;
    }

    fn peek_char(&self) -> char {
        if self.input.len() <= self.read_pos {
            return '\0';
        };

        self.input.chars().nth(self.read_pos).expect(
            format!(
                "couldn't read next token as the lexer's read_pos {} is out of bounds.",
                self.read_pos,
            )
            .as_str(),
        )
    }
}

fn read_dynamic_token(lexer: &mut Lexer) -> TokenType {
    let mut identifier = String::new();
    while lexer.char.is_alphanumeric() || lexer.char == '_' {
        identifier.push(lexer.char);
        lexer.read_char();
    }

    identifier.into()
}

fn skip_white_space(lexer: &mut Lexer) {
    while lexer.char == ' ' || lexer.char == '\t' || lexer.char == '\n' || lexer.char == '\r' {
        lexer.read_char()
    }
}

#[cfg(test)]
mod tests {
    use crate::lexing::token::TokenType;

    use super::*;

    #[test]
    fn test_next_token() {
        let input = r#"
        let one = 1;
        let two = 2;
        let add = fn(x, y) {
            x + y;
        };
        let result = add(one, two);
        !-/*3;
        4 < 5 > 6;

        if (7 < 8) {
            return true;
        } else {
            return false;
        }

        9 == 9;
        9 != 10;
        "#;
        let expectation = [
            TokenType::Let,
            TokenType::Ident("one".to_owned()),
            TokenType::Assign,
            TokenType::Int(1),
            TokenType::Semicolon,
            TokenType::Let,
            TokenType::Ident("two".to_owned()),
            TokenType::Assign,
            TokenType::Int(2),
            TokenType::Semicolon,
            TokenType::Let,
            TokenType::Ident("add".to_owned()),
            TokenType::Assign,
            TokenType::Function,
            TokenType::LParen,
            TokenType::Ident("x".to_owned()),
            TokenType::Comma,
            TokenType::Ident("y".to_owned()),
            TokenType::RParen,
            TokenType::LBrace,
            TokenType::Ident("x".to_owned()),
            TokenType::Plus,
            TokenType::Ident("y".to_owned()),
            TokenType::Semicolon,
            TokenType::RBrace,
            TokenType::Semicolon,
            TokenType::Let,
            TokenType::Ident("result".to_owned()),
            TokenType::Assign,
            TokenType::Ident("add".to_owned()),
            TokenType::LParen,
            TokenType::Ident("one".to_owned()),
            TokenType::Comma,
            TokenType::Ident("two".to_owned()),
            TokenType::RParen,
            TokenType::Semicolon,
            TokenType::Bang,
            TokenType::Minus,
            TokenType::Slash,
            TokenType::Asterisk,
            TokenType::Int(3),
            TokenType::Semicolon,
            TokenType::Int(4),
            TokenType::LT,
            TokenType::Int(5),
            TokenType::GT,
            TokenType::Int(6),
            TokenType::Semicolon,
            TokenType::If,
            TokenType::LParen,
            TokenType::Int(7),
            TokenType::LT,
            TokenType::Int(8),
            TokenType::RParen,
            TokenType::LBrace,
            TokenType::Return,
            TokenType::True,
            TokenType::Semicolon,
            TokenType::RBrace,
            TokenType::Else,
            TokenType::LBrace,
            TokenType::Return,
            TokenType::False,
            TokenType::Semicolon,
            TokenType::RBrace,
            TokenType::Int(9),
            TokenType::Eq,
            TokenType::Int(9),
            TokenType::Semicolon,
            TokenType::Int(9),
            TokenType::NotEq,
            TokenType::Int(10),
            TokenType::Semicolon,
            TokenType::Eof,
        ];

        let mut lexer = Lexer::new(input.to_string());
        let mut idx = 0;
        while lexer.pos < lexer.input.len() {
            let token = lexer.next_token();
            let expected_token = &expectation[idx];

            assert_eq!(
                token, *expected_token,
                "Token {:?} does not match expectation {:?}",
                token, expected_token
            );
            idx += 1;
        }
    }
}
