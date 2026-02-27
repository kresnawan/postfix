#[derive(Debug, Clone, Copy)]
enum Token {
    Number(i32),
    Operand(OperandType),
}

#[derive(Debug, Clone, Copy)]
enum OperandType {
    Multiply,
    Divide,
    Add,
    Subtract,
}

fn main() {
    let tokens = tokenize("3 + 1 / 2 * 10");
    let postfix = postfixer(&tokens);

    println!("{:?}", postfix);
}

fn precedence(arg: OperandType) -> u8 {
    match arg {
        OperandType::Add | OperandType::Subtract => 1,
        OperandType::Multiply | OperandType::Divide => 2,
    }
}

fn postfixer(arg: &Vec<Token>) -> Vec<Token> {
    let mut res: Vec<Token> = Vec::new();
    let mut stack: Vec<Token> = Vec::new();

    for i in arg {
        match i {
            Token::Number(_) => {
                res.push(*i);
            }
            Token::Operand(current) => {
                if stack.len() == 0 {
                    stack.push(*i)
                } else {
                    let last_token_in_stack = stack[stack.len() - 1];
                    let last_token_operand = match last_token_in_stack {
                        Token::Operand(n) => n,
                        _ => panic!("Number must not be in stack"),
                    };
                    if precedence(*current) <= precedence(last_token_operand) {
                        stack.pop().unwrap();
                        stack.push(*i);
                        res.push(last_token_in_stack);
                    } else {
                        stack.push(*i);
                    }
                }
            }
        }
    }

    while let Some(n) = stack.pop() {
        res.push(n);
    }

    res
}

fn sanitize_whitespace(str: &str) -> String {
    let vec: Vec<&str> = str.trim().split(" ").collect();
    let mut res = String::new();

    for i in vec {
        res.push_str(i);
    }

    res
}

fn tokenize(arg: &str) -> Vec<Token> {
    let str = sanitize_whitespace(arg);
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

                let c_as_number: Result<i32, _> = c_as_string.parse();
                match c_as_number {
                    Ok(num) => {
                        res.push(Token::Number(num));
                    }
                    Err(_) => {
                        panic!("Error parsing string");
                    }
                }
            }
            '*' => {
                res.push(Token::Operand(OperandType::Multiply));
            }
            '/' => {
                res.push(Token::Operand(OperandType::Divide));
            }
            '+' => {
                res.push(Token::Operand(OperandType::Add));
            }
            '-' => {
                res.push(Token::Operand(OperandType::Subtract));
            }
            _ => panic!("Char must valid"),
        }
    }

    return res;
}
