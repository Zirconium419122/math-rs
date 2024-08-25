use std::collections::HashMap;

#[derive(Debug)]
pub enum Expression<T> {
    Variable(char),
    Constant(T),
    Addition(Box<Expression<T>>, Box<Expression<T>>),
    Subtraction(Box<Expression<T>>, Box<Expression<T>>),
    Multiplication(Box<Expression<T>>, Box<Expression<T>>),
    Division(Box<Expression<T>>, Box<Expression<T>>),
    // Add more operations as needed
}

impl<T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + std::ops::Div<Output = T> + Default + Copy> Expression<T> {
    pub fn evaluate(&self, variable_values: &HashMap<char, T>) -> T {
        match self {
            Expression::Variable(c) => *variable_values.get(c).unwrap_or(&T::default()),
            Expression::Constant(value) => *value as T,
            Expression::Addition(lhs, rhs) => lhs.evaluate(variable_values) + rhs.evaluate(variable_values),
            Expression::Subtraction(lhs, rhs) => lhs.evaluate(variable_values) - rhs.evaluate(variable_values),
            Expression::Multiplication(lhs, rhs) => lhs.evaluate(variable_values) * rhs.evaluate(variable_values),
            Expression::Division(lhs, rhs) => lhs.evaluate(variable_values) / rhs.evaluate(variable_values),
        }
    }
}
