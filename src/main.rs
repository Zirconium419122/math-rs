mod expression;
mod parser;

use std::collections::HashMap;
use parser::Parser;
use expression::Expression;

// Token representing a part of the input expression
#[derive(Debug, Clone, PartialEq)]
enum Token {
    Variable(char),
    Constant(i32),
    Plus,
    Minus,
    Asterisk,
    OpenParenthesis,
    CloseParenthesis,
}

// Tokenize the input string
fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut iter = input.chars().peekable();

    while let Some(&ch) = iter.peek() {
        match ch {
            'a'..='z' | 'A'..='Z' => {
                tokens.push(Token::Variable(ch));
                iter.next();
            }
            '0'..='9' => {
                let mut value = 0;
                while let Some(&digit) = iter.peek() {
                    if let Some(d) = digit.to_digit(10) {
                        value = value * 10 + d as i32;
                        iter.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Constant(value));
            }
            '+' => {
                tokens.push(Token::Plus);
                iter.next();
            }
            '-' => {
                tokens.push(Token::Minus);
                iter.next();
            }
            '*' => {
                tokens.push(Token::Asterisk);
                iter.next();
            }
            '(' => {
                tokens.push(Token::OpenParenthesis);
                iter.next();
            }
            ')' => {
                tokens.push(Token::CloseParenthesis);
                iter.next();
            }
            _ => {
                // Ignore other characters
                iter.next();
            }
        }
    }

    tokens
}

fn generate_combinations(variables: &[char], max_value: i32) -> Vec<HashMap<char, i32>> {
    let mut combinations = Vec::new();
    let mut current_combination = HashMap::new();

    for variable in variables {
        current_combination.insert(*variable, 0);
    }

    loop {
        combinations.push(current_combination.clone());

        let mut carry = 1;

        for variable in variables {
            if let Some(value) = current_combination.get_mut(variable) {
                *value += carry;
                
                if *value > max_value {
                    *value = 0;
                } else {
                    carry = 0;
                    break;
                }
            }
        }
        
        // If carry is bigger then 0 after iterating through all variables,
        // all variables have rolled over to 0 and all combinations generated.
        // Thus, the loop breaks.
        if carry > 0 {
            break;
        }
    }
    combinations
}

fn brute_force_solver(equation: &Expression, variable_names: &[char], target: i32) {
    let max_value = 20;
    let combinations = generate_combinations(variable_names, max_value);

    for variable_values in combinations {
        let result = equation.evaluate(&variable_values);

        if result == target {
            println!("Solution found: {:?}", variable_values);
            return;
        }
    }

    println!("No solution found.");
}

fn main() {
    // Exapmle: 13a + 7b + 1 = 245
    let input = "13 * a + 7 * b + 1";
    let tokens = tokenize(input);
    let mut parser = Parser::new(&tokens);

    if let Some(expression) = parser.parse_expression() {
        println!("Parsed Expression: {:?}", expression);
        
        // Example variable names (modify as needed)
        let variable_names = vec!['a', 'b'];
        
        // Target value (modify as needed)
        let target = 245;
        
        // Solve using brute force
        brute_force_solver(&expression, &variable_names, target);
    } else {
        println!("Failed to parse expression.");
    }
}
