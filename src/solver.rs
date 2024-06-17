use std::collections::HashMap;

use crate::expression::Expression;
use crate::parser::Parser;
use crate::token::{tokenize, Token};

pub trait Solver<T> {
    fn solve(input: T) -> Result<(), String>;
    fn solve_from_self(&self) -> Result<(), String>;
}

pub struct BruteForce {
    equation: Expression,
    target_expression: Expression,
    tokens: Vec<Token>,
    max_value: i32,
}

impl BruteForce {
    pub fn new(tokens: Vec<Token>, max_value: i32) -> Result<Self, String> {
        let mut parser = Parser::new(&tokens);

        match parser.parse_equation() {
            Some((left, right)) => {
                println!("Parsed Equation: {:?} = {:?}", left, right);

                Ok(Self {
                    equation: left,
                    target_expression: right,
                    tokens,
                    max_value,
                })
            },
            None => Err("Failed to parse expression.".to_string()),
        }
    }

    fn generate_combinations(&self) -> Vec<HashMap<char, i32>> {
        let mut combinations = Vec::new();
        let mut current_combination = HashMap::new();

        let variable_names: Vec<char> = self
            .tokens
            .iter()
            .filter_map(|t| t.get_variable_char())
            .collect();

        let _ = variable_names
            .iter()
            .for_each(|variable_name| {
                current_combination.insert(*variable_name, 0);
            });
            
        loop {
            combinations.push(current_combination.clone());

            let mut carry = 1;

            for variable_name in &variable_names {
                if let Some(value) = current_combination.get_mut(variable_name) {
                    *value += carry;

                    if *value > self.max_value {
                        *value = 0;
                    } else {
                        carry = 0;
                        break;
                    }
                }
            }

            if carry > 0 {
                break;
            }
        }

        combinations
    }
}

impl Solver<&str> for BruteForce {
    fn solve(input: &str) -> Result<(), String> {
        let tokens = tokenize(input);

        let solver = BruteForce::new(tokens, 20).unwrap();

        let combinations = solver.generate_combinations();

        let mut solution_found = false;

        for variable_values in combinations {
            let left_result = solver.equation.evaluate(&variable_values);
            let right_result = solver.target_expression.evaluate(&variable_values);

            if left_result == right_result {
                println!("Solution found: {:?}", variable_values);

                solution_found = true;
            }
        }

        if solution_found {
            return Ok(());
        }

        Err("No solution found.".to_string())
    }

    fn solve_from_self(&self) -> Result<(), String> {
        let combinations = self.generate_combinations();

        let mut solution_found = false;

        for variable_values in combinations {
            let left_result = self.equation.evaluate(&variable_values);
            let right_result = self.target_expression.evaluate(&variable_values);

            if left_result == right_result {
                println!("Solution found: {:?}", variable_values);

                solution_found = true;
            }
        }

        if solution_found {
            return Ok(());
        }

        Err("No solution found.".to_string())
    }
}
