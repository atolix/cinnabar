use crate::parser::Operator;
use crate::parser::AST;

pub fn evaluate(ast: &AST) -> f64 {
    match ast {
        AST::Number(n) => *n,
        AST::BinaryOp(left, op, right) => {
            let left_value = evaluate(&*left);
            let right_value = evaluate(&*right);
            match op {
                Operator::Plus => left_value + right_value,
                Operator::Minus => left_value - right_value,
                Operator::Multiply => left_value * right_value,
                Operator::Divide => left_value / right_value,
            }
        }
        AST::Print(inner) => {
            match &**inner {
                AST::String(s) => {
                    println!("{}", s);
                }
                _ => {
                    let value = evaluate(&*inner);
                    println!("{}", value);
                }
            }
            0.0
        }
        AST::String(_) => {
            panic!("String should not be evaluated");
        }
    }
}
