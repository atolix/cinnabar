use crate::tokenizer::Token;

#[derive(Debug, Clone)]
pub enum AST {
    Number(f64),
    BinaryOp(Box<AST>, Operator, Box<AST>),
}

#[derive(Debug, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

pub fn parse(mut tokens: &[Token]) -> AST {
    let (mut parsed_tokens, rest_tokens) = parse_term(tokens);
    tokens = rest_tokens;

    while !tokens.is_empty() {
        match tokens.split_first() {
            Some((Token::Operator(op), rest_tokens)) => {
                let (right, rest_tokens) = parse_term(rest_tokens);
                parsed_tokens = AST::BinaryOp(Box::new(parsed_tokens), op.clone(), Box::new(right));
                tokens = rest_tokens;
            }
            _ => break,
        }
    }

    if tokens.is_empty() {
        parsed_tokens
    } else {
        panic!("Unexpected token");
    }
}

fn parse_primary(tokens: &[Token]) -> (AST, &[Token]) {
    match tokens.split_first() {
        Some((Token::Number(value), rest_tokens)) => (AST::Number(*value), rest_tokens),
        _ => panic!("Expected a number"),
    }
}

fn parse_term(tokens: &[Token]) -> (AST, &[Token]) {
    let (mut parsed_tokens, mut tokens) = parse_primary(tokens);

    while let Some((Token::Operator(Operator::Multiply), rest_tokens)) = tokens.split_first() {
        let (right, new_tokens) = parse_primary(rest_tokens);
        parsed_tokens = AST::BinaryOp(Box::new(parsed_tokens), Operator::Multiply, Box::new(right));
        tokens = new_tokens;
    }

    (parsed_tokens, tokens)
}
