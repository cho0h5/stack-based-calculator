extern crate stack_based_calculator;
use std::io;

use stack_based_calculator::tokenizer::tokenize;
use stack_based_calculator::converter::convert_to_postfix;
use stack_based_calculator::calculator::calculate;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let tokenized = tokenize(input);

    let converted = convert_to_postfix(tokenized);

    let calculated = calculate(converted);
    println!("result: {}", calculated)
}
