mod parser;
mod tokenizer;

use parser::parse;
use tokenizer::tokenize;

fn main() {
    let input = "1 + 5 * 4";
    let tokens = tokenize(&input).unwrap();
    let ast = parse(&tokens);
    println!("{:?}", ast);
}
