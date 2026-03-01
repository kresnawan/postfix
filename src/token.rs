use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
pub enum Token {
    Number(f64),
    Operator(OperatorType),
    Delimiter(DelimType),
}

#[derive(Debug, Clone, Copy)]
pub enum OperatorType {
    Multiply,
    Divide,
    Add,
    Subtract,
    Caret,
    StartCaret,
}

#[derive(Debug, Clone, Copy)]
pub enum DelimType {
    OpenParen,
    CloseParen,
}
