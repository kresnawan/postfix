use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
pub enum Token {
    Number(i32),
    Operator(OperatorType),
    Delimiter(DelimType),
}

impl Token {
    pub fn get_delim_type(&self) -> Option<DelimType> {
        match *self {
            Token::Delimiter(n) => Some(n),
            _ => None,
        }
    }

    // pub fn get_op_type(&self) -> Option<OperatorType> {
    //     match *self {
    //         Token::Operator(n) => Some(n),
    //         _ => None,
    //     }
    // }
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
