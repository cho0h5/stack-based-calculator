use std::collections::VecDeque;

use super::Operator;
use super::Token;

pub fn convert_to_postfix(tokenized: Vec<Token>) -> Vec<Token> {
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
