use crate::token::{Token, TokenType};

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
            _ => super::error(self.line, "Unexpected character.").unwrap(),
        }
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
                assert_eq!(&test_value.replace("\"", ""), value);
            }
        }
    }
}
