use std::collections::HashMap;
use std::sync::OnceLock;
use crate::token::{Token, TokenType};
use crate::utils;

static KEYWORDS: OnceLock<HashMap<&str, TokenType>> = OnceLock::new();

fn get_keyword_token(literal: &str) -> Option<&TokenType> {
    let keywords = KEYWORDS.get_or_init(|| {
        let mut map = HashMap::new();
        map.insert("and", TokenType::And);
        map.insert("class", TokenType::Class);
        map.insert("else", TokenType::Else);
        map.insert("false", TokenType::False);
        map.insert("for", TokenType::For);
        map.insert("fun", TokenType::Fun);
        map.insert("if", TokenType::If);
        map.insert("nil", TokenType::Nil);
        map.insert("or", TokenType::Or);
        map.insert("print", TokenType::Print);
        map.insert("return", TokenType::Return);
        map.insert("super", TokenType::Super);
        map.insert("this", TokenType::This);
        map.insert("true", TokenType::True);
        map.insert("var", TokenType::Var);
        map.insert("while", TokenType::While);
        map
    });
    keywords.get(literal)
}

pub struct Scanner<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: usize,
    tokens: Vec<Token>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            line: 1,
            tokens: Vec::new(),
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new(TokenType::Eof, "", self.line));
        &self.tokens
    }

    fn scan_token(&mut self) {
        let char = self.advance();
        match char {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.matches('=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang);
                }
            }
            '=' => {
                if self.matches('=') {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal);
                }
            }
            '<' => {
                if self.matches('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less);
                }
            }
            '>' => {
                if self.matches('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater);
                }
            }
            '/' => {
                if self.matches('/') {
                    // A comment goes until the end of the line
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' | '\r' | '\t' => {} // Ignore whitespace
            '\n' => self.line += 1,
            '"' => self.scan_string_literal(),
            _ => {
                if char.is_numeric() {
                    self.scan_number_literal();
                } else if utils::is_alpha(char) {
                    self.scan_identifier();
                }
                else {
                    super::error(self.line, "Unexpected character.").unwrap()
                };
            }
        }
    }

    fn scan_identifier(&mut self) {
        while utils::is_alphanumeric(self.peek())  {
            self.advance();
        }

        let value = &self.source[self.start..self.current];
        match get_keyword_token(value) {
            None => self.add_token(TokenType::Identifier(value.to_string())),
            Some(token_type) => self.add_token(token_type.clone())
        }
    }

    fn scan_number_literal(&mut self) {
        while self.peek().is_numeric() {
            self.advance();
        }

        // Look for fractional part
        if self.peek() == '.' && self.peek_next().is_numeric() {
            // Consume the "."
            self.advance();

            while self.peek().is_numeric() {
                self.advance();
            }
        }

        let str_value = &self.source[self.start..self.current];
        let num_value: f64 = str_value.parse().unwrap();
        self.add_token(TokenType::Number(num_value));
    }

    fn scan_string_literal(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            super::error(self.line, "Unterminated string.").unwrap();
            return;
        }

        // The closing "
        self.advance();

        // Trim the surrounding quotes
        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token(TokenType::String(value.to_string()));
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token::new(token_type, text, self.line));
    }

    fn advance(&mut self) -> char {
        let chars = self.source[self.current..self.current + 1]
            .chars()
            .collect::<Vec<char>>();

        self.current += 1;
        match chars.first() {
            None => '\0',
            Some(value) => *value,
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        let chars = self.source[self.current..self.current + 1]
            .chars()
            .collect::<Vec<char>>();

        match chars.first() {
            None => '\0',
            Some(value) => *value,
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        let chars = self.source[self.current + 1..self.current + 2]
            .chars()
            .collect::<Vec<char>>();

        match chars.first() {
            None => '\0',
            Some(value) => *value,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        let chars = self.source[self.current..self.current + 1]
            .chars()
            .collect::<Vec<char>>();

        if chars.first().is_some_and(|c| *c != expected) {
            return false;
        }

        self.current += 1;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advance() {
        let test_value = "print \"Hello, world!\";";
        let mut result = String::new();
        let mut scanner = Scanner::new(test_value);
        while !scanner.is_at_end() {
            let char = scanner.advance();
            result.push(char);
        }
        assert_eq!(test_value, &result);
    }

    #[test]
    fn test_scan_string() {
        let test_value = "\"Hello, world!\"";
        let mut scanner = Scanner::new(test_value);
        let tokens = scanner.scan_tokens();
        assert_eq!(
            2,
            tokens.len(),
            "there should be one string and one EOF token"
        );

        let token = tokens.get(0);
        if let Some(t) = token {
            if let TokenType::String(value) = &t.token_type {
                assert_eq!(&test_value.replace('"', ""), value);
            } else {
                panic!("wrong token type")
            }
        }
    }

    #[test]
    fn test_scan_number() {
        let test_value = 12.34;
        let test_value_str = &test_value.to_string();
        let mut scanner = Scanner::new(test_value_str);
        let tokens = scanner.scan_tokens();
        assert_eq!(
            2,
            tokens.len(),
            "there should be one number and one EOF token"
        );

        let token = tokens.get(0);
        if let Some(t) = token {
            if let TokenType::Number(value) = &t.token_type {
                assert_eq!(test_value, *value);
            } else {
                panic!("wrong token type")
            }
        }
    }

    #[test]
    fn test_scan_identifier() {
        let test_value = "class";
        let mut scanner = Scanner::new(test_value);
        let tokens = scanner.scan_tokens();
        assert_eq!(
            2,
            tokens.len(),
            "there should be one number and one EOF token"
        );

        let token = tokens.get(0);
        if let Some(t) = token {
            assert_eq!(TokenType::Class, t.token_type);
        }
    }
}
