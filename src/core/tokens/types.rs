#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Print,
    StringLiteral(String),
    LeftParen,
    RightParen,
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            line,
        }
    }
}
