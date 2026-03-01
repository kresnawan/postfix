use std::fmt::{Debug, Display};

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

impl Display for OperatorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Multiply => write!(f, "*"),
            Self::Divide => write!(f, "/"),
            Self::Add => write!(f, "+"),
            Self::Subtract => write!(f, "-"),
            Self::Caret => write!(f, "^"),
            _ => write!(f, ""),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DelimType {
    OpenParen,
    CloseParen,
}
