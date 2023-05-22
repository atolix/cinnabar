use crate::parser::Operator;

#[derive(Debug, Clone)]
pub enum Token {
    Number(f64),
    Operator(Operator)
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

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
            '+' => {
                chars.next();
                tokens.push(Token::Operator(Operator::Plus));
            }
            '-' => {
                chars.next();
                tokens.push(Token::Operator(Operator::Minus));
            }
            '*' => {
                chars.next();
                tokens.push(Token::Operator(Operator::Multiply));
            }
            '/' => {
                chars.next();
                tokens.push(Token::Operator(Operator::Divide));
            }
            ' ' => {
                chars.next();
            }
            _ => return Err(format!("Unexpected character: {}", ch)),
        }
    }

    Ok(tokens)
}
