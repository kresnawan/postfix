use std::fmt::{Debug, Display};

#[derive(Debug)]
pub enum Err {
    EmptyArg,
    DivideByZero,
    InvalidChar,
    DanglingOperand,
    UnmatchedBracket,
}

impl Display for Err {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyArg => write!(f, "Operation is empty"),
            Self::DivideByZero => write!(f, "Cannot divide number by zero"),
            Self::InvalidChar => write!(f, "Operation contains invalid char"),
            Self::DanglingOperand => write!(f, "Operand must between two numbers"),
            Self::UnmatchedBracket => write!(f, "One or more brackets are lonely"),
        }
    }
}

impl std::error::Error for Err {}
