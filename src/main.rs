use math_rs::solver::{BruteForce, Solver};

fn main() {
    let input = "13 * a + 7 * b + 1 = 245";

    let _ = BruteForce::solve(input);
}
