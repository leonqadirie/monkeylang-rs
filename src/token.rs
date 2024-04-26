#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenType {
    ASSIGN,
    ASTERISK,
    BANG,
    COMMA,
    EOF,
    FUNCTION,
    GT,
    IDENT(String),
    ILLEGAL,
    INT(usize),
    LBRACE,
    LET,
    LPAREN,
    LT,
    MINUS,
    PLUS,
    RBRACE,
    RPAREN,
    SEMICOLON,
    SLASH,
}

impl From<String> for TokenType {
    fn from(item: String) -> Self {
        match &*item {
            "=" => Self::ASSIGN,
            "*" => Self::ASTERISK,
            "!" => Self::BANG,
            "," => Self::COMMA,
            "" | "\0" => Self::EOF,
            "fn" => Self::FUNCTION,
            ">" => Self::GT,
            "{" => Self::LBRACE,
            "let" => Self::LET,
            "(" => Self::LPAREN,
            "<" => Self::LT,
            "-" => Self::MINUS,
            "+" => Self::PLUS,
            "}" => Self::RBRACE,
            ")" => Self::RPAREN,
            ";" => Self::SEMICOLON,
            "/" => Self::SLASH,
            // TODO: handle integer overflows with grace, e.g., via TryFrom
            text if text.chars().all(|c| c.is_digit(10)) => Self::INT(
                text.parse::<usize>()
                    .expect(format!("couldn't parse {} as integer", text).as_str()),
            ),
            text if text.chars().all(|c| c.is_alphabetic() || c == '_') => {
                Self::IDENT(text.to_owned())
            }
            _ => Self::ILLEGAL,
        }
    }
}
