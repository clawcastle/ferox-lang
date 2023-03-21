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
            let token_type_result: Result<TokenType, FeroxError> = if Token::is_always_single_character_token(c) && let Ok(token_type) = TokenType::try_from(c) {
                Ok(token_type)
            } else if Token::is_always_single_or_double_character_token(c) {
                match c {
                    '!' => Ok(if self.match_current('=') {TokenType::BangEqual} else  {TokenType::Bang }),
                    '=' => Ok(if self.match_current('=') {TokenType::EqualEqual } else  {TokenType::Equal }),
                    '<' => Ok(if self.match_current('=') {TokenType::LessEqual } else  {TokenType::Less }),
                    '>' => Ok(if self.match_current('=') {TokenType::GreaterEqual} else  {TokenType::Greater }),
                    _ => Err(FeroxError::SyntaxError {
                        error_description: "Unexpected character".to_owned(),
                        line_number: self.line_number,
                    })
                }
            } else if c == '/' {
                if self.match_current('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance()
                    }

                    // Handle comment
                } else {
                    Ok(TokenType::Slash)
                }
            } else {
                Err(FeroxError::SyntaxError {
                    error_description: "Unexpected character".to_owned(),
                    line_number: self.line_number,
                })
            };

            token_type_result.map_or_else(
                |err| self.errors.push(err),
                |token_type| {
                    tokens.push(Token::new(
                        token_type,
                        self.source[self.start..self.current].iter().collect(),
                        self.line_number,
                    ))
                },
            );
        }
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

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
