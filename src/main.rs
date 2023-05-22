mod parser;
mod tokenizer;
mod evaluator;

use parser::parse;
use tokenizer::tokenize;
use evaluator::evaluate;

fn main() {
    let input = "1 + 5 * 4 * 5 / 4";
    let tokens = tokenize(&input).unwrap();
    let ast = parse(&tokens);
    let result: f64 = evaluate(&ast);
    println!("{:?}", result);
}
