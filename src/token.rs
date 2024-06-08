// Token representing a part of the input expression
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Variable(char),
    Constant(i32),
    Plus,
    Minus,
    Asterisk,
    OpenParenthesis,
    CloseParenthesis,
    Equal,
}

impl Token {
    pub fn get_variable_char(&self) -> Option<char> {
        if let Token::Variable(c) = self {
            Some(*c)
        } else {
            None
        }
    }
}

// Tokenize the input string
pub fn tokenize(input: &str) -> Vec<Token> {
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
            '=' => {
                tokens.push(Token::Equal);
                iter.next();
            }
            _ => {
                iter.next();
            }
        }
    }

    tokens
}
