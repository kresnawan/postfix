use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
pub enum Token {
    Number(i32),
    Operator(OperatorType),
    Delimiter(DelimType),
}

#[derive(Debug, Clone, Copy)]
pub enum OperatorType {
    Multiply,
    Divide,
    Add,
    Subtract,
}

#[derive(Debug, Clone, Copy)]
pub enum DelimType {
    OpenParen,
    CloseParen,
}
