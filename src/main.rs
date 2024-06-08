use math_rs::token::tokenize;
use math_rs::solver::{BruteForce, Solver};

fn main() {
    let input = "13 * a + 7 * b + 1 = 245";
    let tokens = tokenize(input);

    let solver = BruteForce::new(tokens, 20).unwrap();

    let _ = solver.solve(input);
}
