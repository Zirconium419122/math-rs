mod expression;
mod parser;
mod token;
mod solver;

use expression::Expression;
use parser::Parser;
use token::{tokenize, Token};
use solver::brute_force_solver;

fn main() {
    // Exapmle: 13a + 7b + 1 = 245
    let input = "13 * a + 7 * b + 1";
    let tokens = tokenize(input);
    let mut parser = Parser::new(&tokens);

    if let Some(expression) = parser.parse_expression() {
        println!("Parsed Expression: {:?}", expression);

        // Target value (modify as needed)
        let target = 245;

        // Solve using brute force
        brute_force_solver(
            &expression,
            &tokens,
            target,
        );
    } else {
        println!("Failed to parse expression.");
    }
}
