#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenType {
    Assign,
    Asterisk,
    Bang,
    Comma,
    Else,
    Eof,
    Eq,
    False,
    Function,
    GT,
    Ident(String),
    If,
    Illegal,
    Int(usize),
    LBrace,
    Let,
    LParen,
    LT,
    Minus,
    NotEq,
    Plus,
    RBrace,
    Return,
    RParen,
    Semicolon,
    Slash,
    True,
}

impl From<String> for TokenType {
    fn from(item: String) -> Self {
        match &*item {
            "=" => Self::Assign,
            "*" => Self::Asterisk,
            "!" => Self::Bang,
            "," => Self::Comma,
            "else" => Self::Else,
            "" | "\0" => Self::Eof,
            "==" => Self::Eq,
            "false" => Self::False,
            "fn" => Self::Function,
            ">" => Self::GT,
            "if" => Self::If,
            "{" => Self::LBrace,
            "let" => Self::Let,
            "(" => Self::LParen,
            "<" => Self::LT,
            "-" => Self::Minus,
            "!=" => Self::NotEq,
            "+" => Self::Plus,
            "}" => Self::RBrace,
            "return" => Self::Return,
            ")" => Self::RParen,
            ";" => Self::Semicolon,
            "/" => Self::Slash,
            "true" => Self::True,
            // TODO: handle integer overflows with grace, e.g., via TryFrom
            text if text.chars().all(|c| c.is_digit(10)) => Self::Int(
                text.parse::<usize>()
                    .expect(format!("couldn't parse {} as integer", text).as_str()),
            ),
            text if text.chars().all(|c| c.is_alphabetic() || c == '_') => {
                Self::Ident(text.to_owned())
            }
            _ => Self::Illegal,
        }
    }
}
