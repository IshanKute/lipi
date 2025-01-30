use crate::core::tokens::Token;
use std::error::Error;

pub trait Lexer {
    fn scan_tokens(&mut self) -> Result<Vec<Token>, Box<dyn Error>>;
}
