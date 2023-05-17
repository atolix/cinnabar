mod tokenizer;
use tokenizer::tokenize;

fn main() {
    let input = "2 + 3 * 4 - (1 + 2) / 3";
    let tokens = tokenize(input);
    println!("{:?}", tokens);
}
