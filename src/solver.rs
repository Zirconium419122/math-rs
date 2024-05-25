use crate::{Token, Expression};
use std::collections::HashMap;

pub fn generate_combinations(variables: &[Token], max_value: i32) -> Vec<HashMap<char, i32>> {
    let mut combinations = Vec::new();
    let mut current_combination = HashMap::new();

    let variable_names: Vec<char> = variables.into_iter().filter_map(|t| t.get_variable_char()).collect();

    for variable_name in &variable_names {
        current_combination.insert(*variable_name, 0);
    }

    loop {
        combinations.push(current_combination.clone());

        let mut carry = 1;

        for variable_name in &variable_names {
            if let Some(value) = current_combination.get_mut(variable_name) {
                *value += carry;

                if *value > max_value {
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

pub fn brute_force_solver(equation: &Expression, variables: &[Token], target: i32) {
    let max_value = 20;
    let combinations = generate_combinations(variables, max_value);

    for variable_values in combinations {
        let result = equation.evaluate(&variable_values);

        if result == target {
            println!("Solution found: {:?}", variable_values);
            return;
        }
    }

    println!("No solution found.");
}
