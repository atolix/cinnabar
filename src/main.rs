mod evaluator;
mod parser;
mod tokenizer;

use evaluator::evaluate;
use parser::parse;
use std::io;
use tokenizer::tokenize;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    let tokens = tokenize(&input.trim().to_string()).unwrap();
    println!("{:?}", tokens);
    let ast = parse(&tokens);
    println!("{:?}", ast);
    let result: f64 = evaluate(&ast);
    println!("{:?}", result);
}
