use std::io;

#[derive(Debug)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    OpeningPerenthesis,
    ClosingPerenthesis,
}

#[derive(Debug)]
enum Token {
    Number(i64),
    Operator(Operator, u32),
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut input = input.split_whitespace();
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
            result1.push(Token::Operator(Operator::OpeningPerenthesis, 0));
        } else if token == ")" {
            result1.push(Token::Operator(Operator::ClosingPerenthesis, 0));
        } else {
            result1.push(Token::Number(token.parse::<i64>().unwrap()));
        }
    }
    println!("{:?}", result1);
}
