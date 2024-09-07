use math_rs::solver::*;

#[test]
fn test_addition_solve() {
    let input = "1 + x = 3";
    assert!(BruteForce::solve(input).is_ok());
}

#[test]
fn test_subtraction_solve () {
    let input = "1 - x + 3 = 0";
    assert!(BruteForce::solve(input).is_ok());
}

#[test]
fn test_multiplication_solve () {
    let input = "5 * x + 4 = 24";
    assert!(BruteForce::solve(input).is_ok());
}

#[test]
fn test_juxtaposition_solve () {
    let input = "13a + 7b + 1 = 245";
    assert!(BruteForce::solve(input).is_ok());
}

#[test]
fn test_division_solve () {
    let input = "x / 2 + 5 = 7";
    assert!(BruteForce::solve(input).is_ok());
}

#[test]
fn test_exponent_solve () {
    let input = "2^x = 16";
    assert!(BruteForce::solve(input).is_ok());
}

