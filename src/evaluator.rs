use crate::parser::Operator;
use crate::parser::AST;

pub fn evaluate(ast: &AST) -> f64 {
    match ast {
        AST::Number(n) => *n,
        AST::BinaryOp(left, op, right) => {
            let left_value = evaluate(&*left);
            let right_value = evaluate(&*right);
            match op {
                Operator::Add => left_value + right_value,
                Operator::Subtract => left_value - right_value,
                Operator::Multiply => left_value * right_value,
                Operator::Divide => left_value / right_value,
            }
        }
    }
}
