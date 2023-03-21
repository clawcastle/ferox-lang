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

    pub fn is_always_single_character_token(c: char) -> bool {
        const ALWAYS_SINGLE_CHARACTER_TOKEN_CHARS: [char; 10] =
            ['(', ')', '{', '}', ',', '.', '-', '+', ';', '*'];

        ALWAYS_SINGLE_CHARACTER_TOKEN_CHARS.contains(&c)
    }

    pub fn is_always_single_or_double_character_token(c: char) -> bool {
        const ALWAYS_SINGLE_OR_DOUBLE_CHARACTER_TOKEN_CHARS: [char; 4] = ['!', '=', '<', '>'];

        ALWAYS_SINGLE_OR_DOUBLE_CHARACTER_TOKEN_CHARS.contains(&c)
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
    Identifier { identifier: String },
    String { value: String },
    Number { value: f64 },
    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    FOr,
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
