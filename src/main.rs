mod parser;
mod tokenizer;
mod evaluator;

use parser::parse;
use tokenizer::tokenize;
use evaluator::evaluate;

fn main() {
    let input = "( 2 + 5 + 7) - ((3 + 2) * 4 / 5)";
    let tokens = tokenize(input).unwrap();
    println!("{:?}", tokens);
    let ast = parse(&tokens);
    println!("{:?}", ast);
    let result: f64 = evaluate(&ast);
    println!("{:?}", result);
}
