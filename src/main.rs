mod err;
mod token;

use err::Err;
use token::{DelimType, OperatorType, Token};

fn main() {
    let postfix = match postfixer("3 * 2 ^ 3 ^ 1 + 4 * 3 ^ 2 ^ 1 * 4") {
        Ok(n) => n,
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    };

    println!("{:?}", postfix);

    println!("{:?}", evaluate(postfix).unwrap());
}

fn precedence(arg: &Token) -> Option<u8> {
    match arg {
        Token::Operator(n) => match n {
            OperatorType::Add | OperatorType::Subtract => Some(1),
            OperatorType::Multiply | OperatorType::Divide => Some(2),
            OperatorType::Caret => Some(3),
            _ => Some(0),
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
                let right = res.pop().unwrap();
                let left = res.pop().unwrap();

                result = match o {
                    OperatorType::Multiply => left * right,
                    OperatorType::Divide => {
                        if left == 0_f64 || right == 0_f64 {
                            return Err(Err::DivideByZero);
                        }
                        left / right
                    }
                    OperatorType::Add => left + right,
                    OperatorType::Subtract => left - right,
                    OperatorType::Caret => left.powf(right),
                    _ => return Err(Err::InvalidPostfix),
                };

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

    println!("Evaluate: {:?}", res);

    if res.len() > 1 {
        return Err(Err::MissingOperator);
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
            Token::Operator(op) => {
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

                if let Token::Operator(OperatorType::Caret) = last_token_in_stack {
                    if let OperatorType::Caret = op {
                        stack.push(i);
                        continue;
                    }
                    while let Some(Token::Operator(n)) = stack.pop() {
                        if let OperatorType::StartCaret = n {
                            break;
                        }
                        res.push(Token::Operator(OperatorType::Caret));
                    }
                } else if let OperatorType::Caret = op {
                    stack.push(Token::Operator(OperatorType::StartCaret));
                    stack.push(i);
                    continue;
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
    let mut paren_hold: i32 = 0;

    while let Some(c) = iter.next() {
        match c {
            c if c.is_ascii_digit() => {
                let mut c_as_string = c.to_string();

                while let Some(&next) = iter.peek() {
                    if next.is_ascii_digit() {
                        c_as_string.push(iter.next().unwrap());
                    } else if next == '.' {
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
            '^' => {
                res.push(Token::Operator(OperatorType::Caret));
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
