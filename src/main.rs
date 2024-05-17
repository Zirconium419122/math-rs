use std::collections::HashMap;

#[derive(Debug)]
enum Expression {
    Variable(char),
    Constant(i32),
    Addition(Box<Expression>, Box<Expression>),
    Subtraction(Box<Expression>, Box<Expression>),
    Multiplication(Box<Expression>, Box<Expression>),
    // Add more operations as needed
}

impl Expression {
    fn evaluate(&self, variable_values: &HashMap<char, i32>) -> i32 {
        match self {
            Expression::Variable(c) => *variable_values.get(c).unwrap_or(&0),
            Expression::Constant(value) => *value,
            Expression::Addition(lhs, rhs) => lhs.evaluate(variable_values) + rhs.evaluate(variable_values),
            Expression::Subtraction(lhs, rhs) => lhs.evaluate(variable_values) - rhs.evaluate(variable_values),
            Expression::Multiplication(lhs, rhs) => lhs.evaluate(variable_values) * rhs.evaluate(variable_values),
        }
    }
}

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

struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
}

impl<'a> Parser<'a> {
    fn new(tokens: & [Token]) -> Parser {
        Parser {
            tokens,
            current: 0,
        }
    }

    fn parse_expression(&mut self) -> Option<Expression> {
        self.parse_addition()
    }

    fn parse_addition(&mut self) -> Option<Expression> {
        let mut left = self.parse_multiplication();

        while let Some(operator) = self.peek() {
            match operator {
                Token::Plus => {
                    self.consume(); // Consume the operator
                    let right = self.parse_multiplication();
                    left = Some(Expression::Addition(Box::new(left.unwrap()), Box::new(right.unwrap())));
                }
                Token::Minus => {
                    self.consume(); // Consume the operator
                    let right = self.parse_multiplication();
                    left = Some(Expression::Subtraction(Box::new(left.unwrap()), Box::new(right.unwrap())));
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
                    left = Some(Expression::Multiplication(Box::new(left.unwrap()), Box::new(right.unwrap())));
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

fn brute_force_solver(equation: &Expression, variable_names: &[char], target: i32) {
    let mut variable_values: HashMap<char, i32> = HashMap::new();

    // Generate all possible combinations of variable values
    let max_value = 20; // You can adjust this based on your problem
    for value in 0..max_value {
        for &variable in variable_names {
            variable_values.insert(variable, value);
        }

        // Evaluate the equation with the current variable values
        let result = equation.evaluate(&variable_values);

        // Check if the result matches the target
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