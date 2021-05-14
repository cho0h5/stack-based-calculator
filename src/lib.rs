#[derive(Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    OpeningPerenthesis,
    ClosingPerenthesis,
}

#[derive(Debug)]
pub enum Token {
    Number(f64),
    Operator(Operator, u32),
}

pub mod tokenizer;
pub mod converter;
pub mod calculator;