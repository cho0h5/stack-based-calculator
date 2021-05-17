#[derive(Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    OpeningPerenthesis,
    ClosingPerenthesis,
}

#[derive(Debug)]
pub enum Token {
    Number(f64),
    Operator(Operator, u32),
}

pub mod calculator;
pub mod converter;
pub mod tokenizer;
