mod expression;
mod parser;
mod token;
mod solver;

use expression::Expression;
use parser::Parser;
use token::{tokenize, Token};
use solver::brute_force_solver;

fn main() {
    let input = "13 * a + 7 * b + 1";
    let tokens = tokenize(input);
    let mut parser = Parser::new(&tokens);

    match parser.parse_expression() {
        Some(expression) => {
            println!("Parsed Expression: {:?}", expression);

            let target = 245;

            brute_force_solver(&expression, &tokens, target);
        }
        None => println!("Failed to parse expression."),
    }
}
