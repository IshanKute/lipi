use crate::core::ast::node::Node;
use crate::core::tokens::Token;
use std::error::Error;

pub trait Parser {
    fn parse(&mut self, tokens: Vec<Token>) -> Result<Node, Box<dyn Error>>;
}
