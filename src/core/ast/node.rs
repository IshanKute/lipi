#[derive(Debug, Clone)]
pub enum Node {
    Print(Box<Node>),
    StringLiteral(String),
    Empty,
}
