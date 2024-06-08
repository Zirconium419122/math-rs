use crate::expression::Expression;
use crate::token::Token;

pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &[Token]) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse_equation(&mut self) -> Option<(Expression, Expression)> {
        let left_side = self.parse_expression()?;

        if self.peek() == Some(&Token::Equal) {
            self.consume(); // Consume the equal sign
            let right_side = self.parse_expression()?;
            Some((left_side, right_side))
        } else {
            None
        }
    }

    pub fn parse_expression(&mut self) -> Option<Expression> {
        self.parse_addition()
    }

    fn parse_addition(&mut self) -> Option<Expression> {
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

    fn parse_multiplication(&mut self) -> Option<Expression> {
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
                _ => break,
            }
        }

        left
    }

    fn parse_primary(&mut self) -> Option<Expression> {
        match self.peek().cloned() {
            Some(Token::Variable(c)) => {
                self.consume(); // Consume the variable token
                Some(Expression::Variable(c))
            }
            Some(Token::Constant(value)) => {
                self.consume(); // Consume the constant token
                Some(Expression::Constant(value))
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

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn consume(&mut self) {
        self.current += 1;
    }

    fn consume_expect(&mut self, expected: Token) -> Option<()> {
        if self.peek() == Some(&expected) {
            self.consume();
            Some(())
        } else {
            None
        }
    }
}
