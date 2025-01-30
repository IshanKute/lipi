use super::traits::Parser;
use crate::core::ast::node::Node;
use crate::core::tokens::{Token, TokenType};
use std::error::Error;

pub struct FunctionParser {
    current: usize,
    tokens: Vec<Token>,
}

impl FunctionParser {
    pub fn new() -> Self {
        FunctionParser { 
            current: 0,
            tokens: Vec::new(),
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn previous(&self) -> Option<&Token> {
        if self.current > 0 {
            self.tokens.get(self.current - 1)
        } else {
            None
        }
    }

    fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        match self.peek() {
            Some(token) => matches!(token.token_type, TokenType::EOF),
            None => true,
        }
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        matches!(&self.peek().unwrap().token_type, t if t == token_type)
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, Box<dyn Error>> {
        if self.check(&token_type) {
            Ok(self.advance().unwrap())
        } else {
            Err(message.into())
        }
    }

    fn parse_print_statement(&mut self) -> Result<Node, Box<dyn Error>> {
        // Expect print keyword
        self.advance(); // consume 'print'

        // Expect left parenthesis
        self.consume(TokenType::LeftParen, "Expect '(' after print.")?;

        // Parse the argument (expecting a string literal)
        let token = self.advance().ok_or("Expect string literal")?;
        let argument = match &token.token_type {
            TokenType::StringLiteral(s) => Node::StringLiteral(s.clone()),
            _ => return Err("Expect string literal in print statement".into()),
        };

        // Expect right parenthesis
        self.consume(TokenType::RightParen, "Expect ')' after string literal.")?;

        Ok(Node::Print(Box::new(argument)))
    }
}

impl Parser for FunctionParser {
    fn parse(&mut self, tokens: Vec<Token>) -> Result<Node, Box<dyn Error>> {
        self.tokens = tokens;
        self.current = 0;

        // For now, we only handle print statements
        match self.peek() {
            Some(token) => match &token.token_type {
                TokenType::Print => self.parse_print_statement(),
                _ => Err("Expected print statement".into()),
            },
            None => Ok(Node::Empty),
        }
    }
}