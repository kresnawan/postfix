use std::fmt::{Debug, Display};

#[derive(Debug, Clone, Copy)]
enum Token {
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

    pub fn get_op_type(&self) -> Option<OperatorType> {
        match *self {
            Token::Operator(n) => Some(n),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum OperatorType {
    Multiply,
    Divide,
    Add,
    Subtract,
}

#[derive(Debug, Clone, Copy)]
enum DelimType {
    OpenParen,
    CloseParen,
}

#[derive(Debug)]
enum Err {
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

fn main() {
    let tokens = match tokenize("8 * (2 + 100) / (9 * 2 + 24)") {
        Ok(n) => n,
        Err(err) => {
            panic!("{}", err);
        }
    };

    let postfix = match postfixer(tokens) {
        Ok(n) => n,
        Err(err) => {
            panic!("{}", err);
        }
    };

    println!("{:?}", postfix);
}

fn precedence(arg: &OperatorType) -> u8 {
    match arg {
        OperatorType::Add | OperatorType::Subtract => 1,
        OperatorType::Multiply | OperatorType::Divide => 2,
        _ => 0,
    }
}

fn postfixer(arg: Vec<Token>) -> Result<Vec<Token>, Err> {
    let mut res: Vec<Token> = Vec::new();
    let mut stack: Vec<Token> = Vec::new();

    let mut arg_iter = arg.iter().peekable();

    while let Some(&i) = arg_iter.next() {
        match i {
            Token::Number(_) => {
                res.push(i);
            }

            Token::Delimiter(current) => match current {
                DelimType::OpenParen => {
                    stack.push(i);
                    continue;
                }
                DelimType::CloseParen => {
                    while let Some(n) = stack.pop() {
                        match n {
                            Token::Delimiter(d) => match d {
                                DelimType::OpenParen => {
                                    break;
                                }

                                _ => {}
                            },
                            _ => {
                                res.push(n);
                            }
                        }
                    }

                    continue;
                }

                _ => {}
            },

            Token::Operator(current) => {
                /* Handling brackets */
                /* Open brackets instantly goes to stack */

                let temp_stack = stack.clone();
                let last_token_in_stack = match temp_stack.last() {
                    Some(n) => n,
                    None => {
                        stack.push(i);
                        continue;
                    }
                };

                if let Some(n) = last_token_in_stack.get_delim_type() {
                    match n {
                        DelimType::OpenParen => {
                            stack.push(i);
                            continue;
                        }
                        _ => {}
                    }
                }

                let last_token_operand = &last_token_in_stack.get_op_type().unwrap();

                if precedence(&current) <= precedence(last_token_operand) {
                    stack.pop().unwrap();
                    stack.push(i);
                    res.push(*last_token_in_stack);
                } else {
                    stack.push(i);
                }
            }
        }
    }

    while let Some(n) = stack.pop() {
        if let Some(_) = n.get_delim_type() {
            return Err(Err::UnmatchedBracket);
        }
        res.push(n);
    }

    Ok(res)
}

fn sanitize_whitespace(str: &str) -> String {
    let vec: Vec<&str> = str.trim().split(" ").collect();
    let mut res = String::new();

    for i in vec {
        res.push_str(i);
    }

    res
}

fn tokenize(arg: &str) -> Result<Vec<Token>, Err> {
    let str = sanitize_whitespace(arg);

    if str.len() == 0 {
        return Err(Err::EmptyArg);
    }

    let mut res: Vec<Token> = Vec::new();

    let mut iter = str.chars().peekable();

    while let Some(c) = iter.next() {
        match c {
            c if c.is_ascii_digit() => {
                let mut c_as_string = c.to_string();

                while let Some(&next) = iter.peek() {
                    if next.is_ascii_digit() {
                        c_as_string.push(iter.next().unwrap());
                    } else {
                        break;
                    }
                }

                let c_as_number: i32 = c_as_string.parse().unwrap();
                res.push(Token::Number(c_as_number));
            }
            '*' => {
                res.push(Token::Operator(OperatorType::Multiply));
            }
            '/' => {
                if let Some(&n) = iter.peek() {
                    if n == '0' {
                        return Err(Err::DivideByZero);
                    }
                }
                res.push(Token::Operator(OperatorType::Divide));
            }
            '+' => {
                res.push(Token::Operator(OperatorType::Add));
            }
            '-' => {
                res.push(Token::Operator(OperatorType::Subtract));
            }
            '(' => {
                res.push(Token::Delimiter(DelimType::OpenParen));
            }
            ')' => {
                res.push(Token::Delimiter(DelimType::CloseParen));
            }
            _ => return Err(Err::InvalidChar),
        }
    }

    return Ok(res);
}
