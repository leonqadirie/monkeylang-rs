use crate::token::TokenType;

struct Lexer {
    input: String,
    pos: usize,
    read_pos: usize,
    char: char,
}

impl Lexer {
    fn new(input: String) -> Self {
        let mut lexer = Self {
            input,
            pos: 0,
            read_pos: 0,
            char: '\0',
        };
        lexer.read_char();

        lexer
    }

    fn next_token(&mut self) -> TokenType {
        skip_white_space(self);

        if self.char.is_ascii_alphabetic() || self.char == '_' {
            read_identifier(self)
        } else if self.char.is_ascii_digit() {
            read_number(self)
        } else {
            let token = self.char.to_string().into();
            self.read_char();

            token
        }
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
}

fn read_identifier(lexer: &mut Lexer) -> TokenType {
    let mut identifier = String::new();
    while lexer.char.is_alphanumeric() || lexer.char == '_' {
        identifier.push(lexer.char);
        lexer.read_char();
    }

    identifier.into()
}

fn read_number(lexer: &mut Lexer) -> TokenType {
    let mut integer: usize = 0;
    while lexer.char.is_ascii_digit() {
        integer *= 10;
        //TODO: handle integer overflow with more grace
        integer = integer.saturating_add(
            lexer
                .char
                .to_digit(10)
                .expect(format!("couldn't convert {} to digit", lexer.char).as_str())
                as usize,
        );
        lexer.read_char();
    }
    TokenType::INT(integer)
}

fn skip_white_space(lexer: &mut Lexer) {
    while lexer.char == ' ' || lexer.char == '\t' || lexer.char == '\n' || lexer.char == '\r' {
        lexer.read_char()
    }
}

#[cfg(test)]
mod tests {
    use crate::token::TokenType;

    use super::*;

    #[test]
    fn test_next_token() {
        let input = "let five = 5;
            let ten = 10;
            let add = fn(x, y) {
                x + y;
            };
            let result = add(five, ten);
        ";
        let expectation = [
            TokenType::LET,
            TokenType::IDENT("five".to_owned()),
            TokenType::ASSIGN,
            TokenType::INT(5),
            TokenType::SEMICOLON,
            TokenType::LET,
            TokenType::IDENT("ten".to_owned()),
            TokenType::ASSIGN,
            TokenType::INT(10),
            TokenType::SEMICOLON,
            TokenType::LET,
            TokenType::IDENT("add".to_owned()),
            TokenType::ASSIGN,
            TokenType::FUNCTION,
            TokenType::LPAREN,
            TokenType::IDENT("x".to_owned()),
            TokenType::COMMA,
            TokenType::IDENT("y".to_owned()),
            TokenType::RPAREN,
            TokenType::LBRACE,
            TokenType::IDENT("x".to_owned()),
            TokenType::PLUS,
            TokenType::IDENT("y".to_owned()),
            TokenType::SEMICOLON,
            TokenType::RBRACE,
            TokenType::SEMICOLON,
            TokenType::LET,
            TokenType::IDENT("result".to_owned()),
            TokenType::ASSIGN,
            TokenType::IDENT("add".to_owned()),
            TokenType::LPAREN,
            TokenType::IDENT("five".to_owned()),
            TokenType::COMMA,
            TokenType::IDENT("ten".to_owned()),
            TokenType::RPAREN,
            TokenType::SEMICOLON,
            TokenType::EOF,
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