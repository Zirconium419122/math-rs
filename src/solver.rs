use std::collections::HashMap;

use crate::token::Token;
use crate::parser::Parser;
use crate::expression::Expression;

pub trait Solver<T> {
    fn solve(&self, input: T) -> Result<(), String>;
}

pub struct BruteForce {
    equation: Expression,
    tokens: Vec<Token>,
    max_value: i32,
    target: i32,
}

impl BruteForce {
    pub fn new(tokens: Vec<Token>, max_value: i32, target: i32) -> Result<Self, String> {
        let mut parser = Parser::new(&tokens);

        match parser.parse_expression() {
            Some(expression) => {
                println!("Parsed Expression: {:?}", expression);

                Ok(Self {
                    equation: expression,
                    tokens,
                    max_value,
                    target,
                })
            },
            None => Err("Failed to parse expression.".to_string()),
        }
    }

    fn generate_combinations(&self) -> Vec<HashMap<char, i32>> {
        let mut combinations = Vec::new();
        let mut current_combination = HashMap::new();
    
        let variable_names: Vec<char> = self.tokens.clone().into_iter().filter_map(|t| t.get_variable_char()).collect();
    
        for variable_name in &variable_names {
            current_combination.insert(*variable_name, 0);
        }
    
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
    fn solve(&self, _input: &str) -> Result<(), String> {
        let combinations = self.generate_combinations();

        for variable_values in combinations {
            let result = self.equation.evaluate(&variable_values);

            if result == self.target {
                println!("Solution found: {:?}", variable_values);
                
                return Ok(());
            }
        }

        Err("No solution found.".to_string())
    }
}
