use crate::tokenizer::Token;

#[derive(Debug, Clone)]
pub enum AST {
    String(String),
    Number(f64),
    BinaryOp(Box<AST>, Operator, Box<AST>),
    Print(Box<AST>)
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
        Some((Token::LParen, rest_tokens)) => parse_within_parenthesis(rest_tokens),
        Some((Token::Print, rest_tokens)) => parse_print(rest_tokens),
        _ => panic!("Expected a number"),
    }
}

fn parse_term(tokens: &[Token]) -> (AST, &[Token]) {
    let (mut parsed_tokens, mut tokens) = parse_primary(tokens);

    while let Some((Token::Operator(op), rest_tokens)) = tokens.split_first() {
        match op {
            Operator::Multiply | Operator::Divide => {
                let (right, new_tokens) = parse_primary(rest_tokens);
                parsed_tokens = AST::BinaryOp(Box::new(parsed_tokens), op.clone(), Box::new(right));
                tokens = new_tokens;
            }
            _ => break,
        }
    }

    (parsed_tokens, tokens)
}

fn parse_print(tokens: &[Token]) -> (AST, &[Token]) {
    match tokens.split_first() {
        Some((Token::LParen, rest_tokens)) => {
            let (ast, rest_tokens) = parse_string(rest_tokens);
            match rest_tokens.split_first() {
                Some((Token::RParen, rest_tokens)) => (AST::Print(Box::new(ast)), rest_tokens),
                _ => panic!("Expected a closing parenthesis"),
            }
        }
        _ => panic!("Expected an opening parenthesis"),
    }
}

fn parse_string(tokens: &[Token]) -> (AST, &[Token]) {
    match tokens.split_first() {
        Some((Token::String(s), rest_tokens)) => (AST::String(s.clone()), rest_tokens),
        _ => panic!("Expected a string"),
    }
}

fn parse_within_parenthesis(tokens: &[Token]) -> (AST, &[Token]) {
    let mut tokens = tokens;
    let mut parenthesis_nesting_level = 1;
    let mut tokens_within_parenthesis = vec![];

    while parenthesis_nesting_level > 0 {
        match tokens.split_first() {
            Some((Token::LParen, rest_tokens)) => {
                parenthesis_nesting_level += 1;
                tokens_within_parenthesis.push(Token::LParen);
                tokens = rest_tokens;
            }
            Some((Token::RParen, rest_tokens)) => {
                parenthesis_nesting_level -= 1;
                if parenthesis_nesting_level > 0 {
                    tokens_within_parenthesis.push(Token::RParen);
                }
                tokens = rest_tokens;
            }
            Some((token, rest_tokens)) => {
                tokens_within_parenthesis.push(token.clone());
                tokens = rest_tokens;
            }
            _ => panic!("Invalid parenthesis"),
        }
    }

    let ast = parse(&tokens_within_parenthesis);

    (ast, tokens)
}

