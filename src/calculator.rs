use std::collections::VecDeque;

use super::Operator;
use super::Token;

pub fn calculate(converted: Vec<Token>) -> f64 {
    let mut converted: VecDeque<Token> = converted.into_iter().collect();
    let mut stack = Vec::new();
    while let Some(token) = converted.pop_front() {
        match token {
            Token::Number(n) => stack.push(n),
            Token::Operator(Operator::Add, _) => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a + b);
            }
            Token::Operator(Operator::Sub, _) => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a + b);
            }
            Token::Operator(Operator::Mul, _) => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a * b);
            }
            Token::Operator(Operator::Div, _) => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a / b);
            }
            _ => (),
        }
    }
    stack.pop().unwrap()
}
