use std::collections::HashMap;

#[derive(Debug)]
pub enum Expression {
    Variable(char),
    Constant(i32),
    Addition(Box<Expression>, Box<Expression>),
    Subtraction(Box<Expression>, Box<Expression>),
    Multiplication(Box<Expression>, Box<Expression>),
    // Add more operations as needed
}

impl Expression {
    pub fn evaluate(&self, variable_values: &HashMap<char, i32>) -> i32 {
        match self {
            Expression::Variable(c) => *variable_values.get(c).unwrap_or(&0),
            Expression::Constant(value) => *value,
            Expression::Addition(lhs, rhs) => lhs.evaluate(variable_values) + rhs.evaluate(variable_values),
            Expression::Subtraction(lhs, rhs) => lhs.evaluate(variable_values) - rhs.evaluate(variable_values),
            Expression::Multiplication(lhs, rhs) => lhs.evaluate(variable_values) * rhs.evaluate(variable_values),
        }
    }
}
