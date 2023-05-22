use crate::parser::Operator;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Token {
    Number(f64),
    Operator(Operator),
    LParen,
    RParen,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    let mut operators = HashMap::new();
    operators.insert('+', Operator::Plus);
    operators.insert('-', Operator::Minus);
    operators.insert('*', Operator::Multiply);
    operators.insert('/', Operator::Divide);

    while let Some(&ch) = chars.peek() {
        match ch {
            '0'..='9' => {
                let mut number = String::new();
                while let Some('0'..='9') = chars.peek() {
                    number.push(chars.next().unwrap());
                }

                if let Some('.') = chars.peek() {
                    number.push(chars.next().unwrap());
                    while let Some('0'..='9') = chars.peek() {
                        number.push(chars.next().unwrap());
                    }
                }

                let number: f64 = number.parse().unwrap();
                tokens.push(Token::Number(number));
            }
            '(' => {
                chars.next();
                tokens.push(Token::LParen);
            }
            ')' => {
                chars.next();
                tokens.push(Token::RParen);
            }
            ' ' => {
                chars.next();
            }
            _ => {
                if let Some(op) = operators.get(&ch) {
                    chars.next();
                    tokens.push(Token::Operator(op.clone()));
                } else {
                    return Err(format!("Unexpected character: {}", ch));
                }
            }
        }
    }

    Ok(tokens)
}
