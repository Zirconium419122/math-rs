use crate::expression::Expression;
use crate::token::Token;

pub struct Parser<'a, T> {
    tokens: &'a [Token<T>],
    current: usize,
}

impl<'a, T: std::cmp::PartialEq + Clone> Parser<'a, T> {
    pub fn new(tokens: &'a [Token<T>]) -> Parser<T> {
        Parser { tokens, current: 0 }
    }

    pub fn parse_equation(&mut self) -> Option<(Expression<T>, Expression<T>)> {
        let left_side = self.parse_expression()?;

        if self.peek() == Some(&Token::Equal) {
            self.consume(); // Consume the equal sign
            let right_side = self.parse_expression()?;
            Some((left_side, right_side))
        } else {
            None
        }
    }

    fn parse_expression(&mut self) -> Option<Expression<T>> {
        self.parse_addition()
    }

    fn parse_addition(&mut self) -> Option<Expression<T>> {
        let mut left = self.parse_multiplication();

        while let Some(operator) = self.peek() {
            match operator {
                Token::Plus => {
                    self.consume(); // Consume the operator
                    let right = self.parse_multiplication();
                    left = Some(Expression::Addition(
                        Box::new(left.unwrap()),
                        Box::new(right.unwrap()),
                    ));
                }
                Token::Minus => {
                    self.consume(); // Consume the operator
                    let right = self.parse_multiplication();
                    left = Some(Expression::Subtraction(
                        Box::new(left.unwrap()),
                        Box::new(right.unwrap()),
                    ));
                }
                _ => break,
            }
        }

        left
    }

    fn parse_multiplication(&mut self) -> Option<Expression<T>> {
        let mut left = self.parse_primary();

        while let Some(operator) = self.peek() {
            match operator {
                Token::Asterisk => {
                    self.consume(); // Consume the operator
                    let right = self.parse_primary();
                    left = Some(Expression::Multiplication(
                        Box::new(left.unwrap()),
                        Box::new(right.unwrap()),
                    ));
                }
                Token::Slash  => {
                    self.consume(); // Consume the operator
                    let right = self.parse_primary();
                    left = Some(Expression::Division(
                        Box::new(left.unwrap()),
                        Box::new(right.unwrap()),
                    ));
                }
                _ => break,
            }
        }

        left
    }

    fn parse_primary(&mut self) -> Option<Expression<T>> {
        match self.peek().cloned() {
            Some(Token::Variable(c)) => {
                self.consume(); // Consume the variable token
                if let Some(Token::Constant(value)) = self.peek().cloned() {
                    self.consume(); // Consume the constant token
                    Some(Expression::Multiplication(
                        Box::new(Expression::Variable(c)),
                        Box::new(Expression::Constant(value)),
                    ))
                } else {
                    Some(Expression::Variable(c))
                }
            }
            Some(Token::Constant(value)) => {
                self.consume(); // Consume the constant token
                if let Some(Token::Variable(c)) = self.peek().cloned() {
                    self.consume(); // Consume the variable token
                    Some(Expression::Multiplication(
                        Box::new(Expression::Constant(value)),
                        Box::new(Expression::Variable(c)),
                    ))
                } else {
                    Some(Expression::Constant(value))
                }
            }
            Some(Token::OpenParenthesis) => {
                self.consume(); // Consume the opening parenthesis
                let expr = self.parse_expression();
                self.consume_expect(Token::CloseParenthesis)?; // Consume the closing parenthesis
                expr
            }
            _ => None,
        }
    }

    fn peek(&self) -> Option<&Token<T>> {
        self.tokens.get(self.current)
    }

    fn consume(&mut self) {
        self.current += 1;
    }

    fn consume_expect(&mut self, expected: Token<T>) -> Option<()> {
        if self.peek() == Some(&expected) {
            self.consume();
            Some(())
        } else {
            None
        }
    }
}
