mod err;
mod token;

use err::Err;
use token::{DelimType, OperatorType, Token};

fn main() {
    let postfix = match postfixer("20 / 5") {
        Ok(n) => n,
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    };

    println!("{:?}", evaluate(postfix).unwrap());
}

fn precedence(arg: &Token) -> Option<u8> {
    match arg {
        Token::Operator(n) => match n {
            OperatorType::Add | OperatorType::Subtract => Some(1),
            OperatorType::Multiply | OperatorType::Divide => Some(2),
        },

        Token::Delimiter(_) => Some(0),
        _ => None,
    }
}

fn evaluate(postfix: Vec<Token>) -> Result<f64, Err> {
    let mut res: Vec<f64> = Vec::new();

    for token in postfix {
        let result: f64;
        match token {
            Token::Operator(o) => {
                match o {
                    OperatorType::Multiply => {
                        result = res[res.len() - 2] * res[res.len() - 1];
                    }
                    OperatorType::Divide => {
                        if res[res.len() - 2] == 0_f64 || res[res.len() - 1] == 0_f64 {
                            return Err(Err::DivideByZero);
                        }
                        result = res[res.len() - 2] / res[res.len() - 1];
                    }
                    OperatorType::Add => {
                        result = res[res.len() - 2] + res[res.len() - 1];
                    }
                    OperatorType::Subtract => {
                        result = res[res.len() - 2] - res[res.len() - 1];
                    }
                }

                for _ in 0..=1 {
                    res.pop().unwrap();
                }
                res.push(result);
            }
            Token::Number(n) => {
                res.push(n);
                continue;
            }
            _ => {
                return Err(Err::InvalidPostfix);
            }
        }
    }

    Ok(res[0])
}

fn postfixer(operation: &str) -> Result<Vec<Token>, Err> {
    let arg = match tokenize(operation) {
        Ok(res) => res,
        Err(e) => {
            return Err(e);
        }
    };

    let mut res: Vec<Token> = Vec::new();
    let mut stack: Vec<Token> = Vec::new();

    let mut arg_iter = arg.iter().peekable();

    while let Some(&i) = arg_iter.next() {
        match i {
            /* Operand handler */
            Token::Number(_) => {
                res.push(i);
            }

            /* Delimiter handler */
            Token::Delimiter(current) => match current {
                DelimType::OpenParen => {
                    stack.push(i);
                    continue;
                }

                DelimType::CloseParen => {
                    while let Some(n) = stack.pop() {
                        if let Token::Delimiter(DelimType::OpenParen) = n {
                            break;
                        } else {
                            res.push(n);
                        }
                    }

                    continue;
                }
            },

            /* Operator handler */
            Token::Operator(_) => {
                /* Checks if there are double operator */
                if let Some(&n) = arg_iter.peek() {
                    if let Token::Operator(_) = n {
                        return Err(Err::DanglingOperator);
                    }
                }

                let temp_stack = stack.clone();
                let last_token_in_stack = match temp_stack.last() {
                    Some(n) => n,
                    None => {
                        stack.push(i);
                        continue;
                    }
                };

                let current_precedence = precedence(&i).unwrap();
                let last_token_precedence = precedence(&last_token_in_stack).unwrap();

                if current_precedence <= last_token_precedence {
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
    let mut paren_hold: u32 = 0;

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

                let c_as_number: f64 = c_as_string.parse().unwrap();
                res.push(Token::Number(c_as_number));
            }
            '*' => {
                res.push(Token::Operator(OperatorType::Multiply));
            }
            '/' => {
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
                paren_hold += 1;
            }
            ')' => {
                res.push(Token::Delimiter(DelimType::CloseParen));
                paren_hold -= 1;
            }
            _ => return Err(Err::InvalidChar),
        }
    }

    if paren_hold != 0 {
        return Err(Err::UnmatchedBracket);
    }

    return Ok(res);
}
