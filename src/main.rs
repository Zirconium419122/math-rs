mod expression;
mod parser;
mod token;
mod solver;

use expression::Expression;
use parser::Parser;
use token::{tokenize, Token};
use solver::{BruteForce, Solver};

fn main() {
    let input = "13 * a + 7 * b + 1";
    let tokens = tokenize(input);

    let solver = BruteForce::new(tokens, 20, 245).unwrap();

    let _ = solver.solve(input);
}
