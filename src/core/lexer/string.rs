use crate::core::tokens::{Token, TokenType};
use crate::lang::Language;
use crate::core::lexer::traits::Lexer;
use std::error::Error;

pub struct SourceLexer<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    language: Language,
}

impl<'a> SourceLexer<'a> {
    pub fn new(source: &'a str, language: Language) -> Self {
        SourceLexer {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            language,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let current_char = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        current_char
    }

    fn add_token(&mut self, token_type: TokenType) {
        let lexeme = &self.source[self.start..self.current];
        self.tokens.push(Token::new(token_type, lexeme.to_string(), self.line));
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() { return false; }
        if self.source.chars().nth(self.current).unwrap() != expected { return false; }
        
        self.current += 1;
        true
    }

    fn string(&mut self) -> Result<(), Box<dyn Error>> {
        while !self.is_at_end() && self.source.chars().nth(self.current).unwrap() != '"' {
            if self.source.chars().nth(self.current).unwrap() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err("Unterminated string.".into());
        }

        // The closing ".
        self.advance();

        // Trim the surrounding quotes
        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token(TokenType::StringLiteral(value.to_string()));
        Ok(())
    }
}

impl<'a> Lexer for SourceLexer<'a> {
    fn scan_tokens(&mut self) -> Result<Vec<Token>, Box<dyn Error>> {
        while !self.is_at_end() {
            self.start = self.current;
            let c = self.advance();

            match c {
                '(' => self.add_token(TokenType::LeftParen),
                ')' => self.add_token(TokenType::RightParen),
                '"' => self.string()?,
                ' ' | '\r' | '\t' => {}, // Ignore whitespace
                '\n' => self.line += 1,
                _ => {
                    if c.is_alphabetic() {
                        // Handle keywords (print/likho)
                        while !self.is_at_end() && 
                              self.source.chars().nth(self.current).unwrap().is_alphanumeric() {
                            self.advance();
                        }
                        
                        let text = &self.source[self.start..self.current];
                        if text == self.language.get_print_keyword() {
                            self.add_token(TokenType::Print);
                        } else {
                            return Err(format!("Unexpected identifier: {}", text).into());
                        }
                    } else {
                        return Err(format!("Unexpected character: {}", c).into());
                    }
                }
            }
        }

        self.tokens.push(Token::new(TokenType::EOF, "".to_string(), self.line));
        Ok(std::mem::take(&mut self.tokens))
    }
}
