#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenType {
    ASSIGN,
    COMMA,
    EOF,
    FUNCTION,
    IDENT(String),
    INT(usize),
    ILLEGAL,
    LBRACE,
    LET,
    LPAREN,
    PLUS,
    RBRACE,
    RPAREN,
    SEMICOLON,
}

impl From<String> for TokenType {
    fn from(item: String) -> Self {
        match &*item {
            "=" => Self::ASSIGN,
            "," => Self::COMMA,
            "" | "\0" => Self::EOF,
            "fn" => Self::FUNCTION,
            "{" => Self::LBRACE,
            "let" => Self::LET,
            "(" => Self::LPAREN,
            "+" => Self::PLUS,
            "}" => Self::RBRACE,
            ")" => Self::RPAREN,
            ";" => Self::SEMICOLON,
            text if text.chars().all(|c| c.is_alphabetic() || c == '_') => {
                Self::IDENT(text.to_owned())
            }
            _ => Self::ILLEGAL,
        }
    }
}
