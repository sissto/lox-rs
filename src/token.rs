use std::fmt::{Display, Formatter, Pointer};

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: &str, line: usize) -> Self {
        Self {
            token_type,
            lexeme: lexeme.to_string(),
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.token_type, self.lexeme)
    }
}

pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier(String),
    String(String),
    Number(f64),

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::LeftParen => f.write_str("("),
            TokenType::RightParen => f.write_str(")"),
            TokenType::LeftBrace => f.write_str("{"),
            TokenType::RightBrace => f.write_str("}"),
            TokenType::Comma => f.write_str(","),
            TokenType::Dot => f.write_str("."),
            TokenType::Minus => f.write_str("-"),
            TokenType::Plus => f.write_str("+"),
            TokenType::Semicolon => f.write_str(";"),
            TokenType::Slash => f.write_str("/"),
            TokenType::Star => f.write_str("*"),
            TokenType::Bang => f.write_str("!"),
            TokenType::BangEqual => f.write_str("!="),
            TokenType::Equal => f.write_str("="),
            TokenType::EqualEqual => f.write_str("=="),
            TokenType::Greater => f.write_str(">"),
            TokenType::GreaterEqual => f.write_str(">="),
            TokenType::Less => f.write_str("<"),
            TokenType::LessEqual => f.write_str("<="),
            TokenType::Identifier(id) => f.write_str(id),
            TokenType::String(str) => f.write_str(str),
            TokenType::Number(num) => f.write_str(&num.to_string()),
            TokenType::And => f.write_str("and"),
            TokenType::Class => f.write_str("class"),
            TokenType::Else => f.write_str("else"),
            TokenType::False => f.write_str("false"),
            TokenType::Fun => f.write_str("fun"),
            TokenType::For => f.write_str("for"),
            TokenType::If => f.write_str("if"),
            TokenType::Nil => f.write_str("nil"),
            TokenType::Or => f.write_str("or"),
            TokenType::Print => f.write_str("print"),
            TokenType::Return => f.write_str("return"),
            TokenType::Super => f.write_str("super"),
            TokenType::This => f.write_str("this"),
            TokenType::True => f.write_str("true"),
            TokenType::Var => f.write_str("var"),
            TokenType::While => f.write_str("while"),
            TokenType::Eof => f.write_str("\\d"),
        }
    }
}
