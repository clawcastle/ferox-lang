use std::collections::HashSet;

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line_number: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line_number: usize) -> Self {
        Self {
            token_type,
            lexeme,
            line_number,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum TokenType {
    // Single character tokens
    LeftParentheses,
    RightParentheses,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
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
    Identifier,
    String { value: String },
    Number { value: f64 },
    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Null,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    // Other
    Eof,
}

impl TokenType {
    pub fn try_keyword_from_str(s: &str) -> Result<Self, &str> {
        match s {
            "and" => Ok(TokenType::And),
            "class" => Ok(TokenType::Class),
            "else" => Ok(TokenType::Else),
            "false" => Ok(TokenType::False),
            "for" => Ok(TokenType::For),
            "fun" => Ok(TokenType::Fun),
            "if" => Ok(TokenType::If),
            "null" => Ok(TokenType::Null),
            "or" => Ok(TokenType::Or),
            "print" => Ok(TokenType::Print),
            "return" => Ok(TokenType::Return),
            "super" => Ok(TokenType::Super),
            "this" => Ok(TokenType::This),
            "true" => Ok(TokenType::True),
            "var" => Ok(TokenType::Var),
            "while" => Ok(TokenType::While),
            _ => Err(s),
        }
    }
}

impl TryFrom<char> for TokenType {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '(' => Ok(TokenType::LeftParentheses),
            ')' => Ok(TokenType::RightParentheses),
            '{' => Ok(TokenType::LeftBrace),
            '}' => Ok(TokenType::RightBrace),
            ',' => Ok(TokenType::Comma),
            '.' => Ok(TokenType::Dot),
            '-' => Ok(TokenType::Minus),
            '+' => Ok(TokenType::Plus),
            ';' => Ok(TokenType::SemiColon),
            '*' => Ok(TokenType::Star),
            '!' => Ok(TokenType::Bang),
            '=' => Ok(TokenType::Equal),
            '<' => Ok(TokenType::Less),
            '>' => Ok(TokenType::Greater),
            _ => Err(()),
        }
    }
}
