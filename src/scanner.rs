use crate::{
    error::FeroxError,
    token::{Token, TokenType},
};

#[derive(Default)]
pub struct Scanner {
    source: Vec<char>,
    start: usize,
    line_number: usize,
    current: usize,
    pub errors: Vec<FeroxError>,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            line_number: 0,
            current: 0,
            start: 0,
            errors: vec![],
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, FeroxError> {
        let mut tokens: Vec<Token> = Vec::new();

        while !self.is_at_end() {
            self.start = self.current;

            self.scan_token(&mut tokens);
        }

        tokens.push(Token::new(TokenType::Eof, String::new(), self.line_number));

        Ok(tokens)
    }

    fn scan_token(&mut self, tokens: &mut Vec<Token>) {
        if let Some(c) = self.advance() {
            if Token::is_always_single_character_token(c) && let Ok(token_type) = TokenType::try_from(c) {
                self.add_token(tokens, token_type);
            } else if Token::is_always_single_or_double_character_token(c) {
                let token_type = match c {
                    '!' => if self.match_current('=') {TokenType::BangEqual} else  {TokenType::Bang },
                    '=' => if self.match_current('=') {TokenType::EqualEqual } else  {TokenType::Equal },
                    '<' => if self.match_current('=') {TokenType::LessEqual } else  {TokenType::Less },
                    '>' => if self.match_current('=') {TokenType::GreaterEqual} else  {TokenType::Greater },
                    _ => unreachable!()
                };

                self.add_token(tokens, token_type);
            } else if c == '/' {
                if self.match_current('/') {
                    while self.peek().is_some() && !self.is_at_end() {
                        _ = self.advance();
                    }

                    // Handle comment
                } else {
                    self.add_token(tokens, TokenType::Slash);
                }
            } else if self.should_ignore(c) {
                if c == '\n' {
                    self.line_number += 1;
                }
            } else {
                self.errors.push(FeroxError::SyntaxError {
                    error_description: "Unexpected character".to_owned(),
                    line_number: self.line_number,
                });
            };
        }
    }

    fn add_token(&self, tokens: &mut Vec<Token>, token_type: TokenType) {
        let token = Token::new(
            token_type,
            self.source[self.start..self.current].iter().collect(),
            self.line_number,
        );

        tokens.push(token);
    }

    fn match_current(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            false
        } else if let Some(c) = self.source.get(self.current) && *c != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn advance(&mut self) -> Option<char> {
        let current_char = self.source.get(self.current);

        self.current += 1;

        current_char.copied()
    }

    fn peek(&self) -> Option<char> {
        let current_char = self.source.get(self.current);

        current_char.copied()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn should_ignore(&self, c: char) -> bool {
        matches!(c, ' ' | '\r' | '\t' | '\n')
    }
}
