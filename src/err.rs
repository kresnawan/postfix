use std::fmt::{Debug, Display};

#[derive(Debug)]
pub enum Err {
    EmptyArg,
    DivideByZero,
    InvalidChar,
    DanglingOperator,
    UnmatchedBracket,
    InvalidPostfix,
    MissingOperator,
}

impl Display for Err {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyArg => write!(f, "Operation is empty"),
            Self::DivideByZero => write!(f, "Cannot divide number by zero"),
            Self::InvalidChar => write!(f, "Operation contains invalid char"),
            Self::DanglingOperator => write!(f, "Operator must between two numbers"),
            Self::UnmatchedBracket => write!(f, "One or more brackets are lonely"),
            Self::InvalidPostfix => write!(
                f,
                "Only operators and numbers are allowed to be inside the postfix"
            ),
            Self::MissingOperator => write!(f, "Operator is missing before the parenthesis"),
        }
    }
}

impl std::error::Error for Err {}
