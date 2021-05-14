use std::collections::VecDeque;
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

    let tokenized = tokenization(input);
    println!("{:?}", tokenized);

    let converted = convert_to_postfix(tokenized);
    println!("{:?}", converted);
}

fn tokenization(str: String) -> Vec<Token> {
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
            result1.push(Token::Operator(Operator::OpeningPerenthesis, 0));
        } else if token == ")" {
            result1.push(Token::Operator(Operator::ClosingPerenthesis, 0));
        } else {
            result1.push(Token::Number(token.parse::<i64>().unwrap()));
        }
    }
    result1
}

fn convert_to_postfix(tokenized: Vec<Token>) -> Vec<Token> {
    let mut result = Vec::new();
    let mut operator_temp = Vec::new();
    let mut tokenized: VecDeque<Token> = tokenized.into_iter().collect();
    loop {
        match tokenized.pop_front() {
            Some(Token::Number(n)) => result.push(Token::Number(n)),
            Some(Token::Operator(Operator::OpeningPerenthesis, p)) => {
                operator_temp.push((Operator::OpeningPerenthesis, p));
            }
            Some(Token::Operator(Operator::ClosingPerenthesis, _)) => loop {
                match operator_temp.pop() {
                    Some((Operator::OpeningPerenthesis, _)) => {
                        break;
                    }
                    Some((top_o, top_p)) => {
                        result.push(Token::Operator(top_o, top_p));
                    }
                    None => (),
                }
            },
            Some(Token::Operator(o, p)) => match operator_temp.pop() {
                Some((top_o, top_p)) => {
                    if top_p < p {
                        result.push(Token::Operator(top_o, top_p));
                        tokenized.push_front(Token::Operator(o, p));
                    } else {
                        operator_temp.push((top_o, top_p));
                        operator_temp.push((o, p));
                    }
                }
                None => {
                    operator_temp.push((o, p));
                }
            },
            None => break,
        }
    }

    while let Some((o, p)) = operator_temp.pop() {
        result.push(Token::Operator(o, p));
    }

    result
}
