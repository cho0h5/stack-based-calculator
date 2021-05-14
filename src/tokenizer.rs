use super::Operator;
use super::Token;

pub fn tokenize(str: String) -> Vec<Token> {
    let mut input = str.split_whitespace();
    let mut result1 = Vec::new();
    while let Some(token) = input.next() {
        if token == "+" {
            result1.push(Token::Operator(Operator::Add, 2));
        } else if token == "-" {
            result1.push(Token::Operator(Operator::Sub, 2));
        } else if token == "*" {
            result1.push(Token::Operator(Operator::Mul, 1));
        } else if token == "/" {
            result1.push(Token::Operator(Operator::Div, 1));
        } else if token == "(" {
            result1.push(Token::Operator(Operator::OpeningPerenthesis, 100));
        } else if token == ")" {
            result1.push(Token::Operator(Operator::ClosingPerenthesis, 100));
        } else {
            result1.push(Token::Number(token.parse::<f64>().unwrap()));
        }
    }
    result1
}
