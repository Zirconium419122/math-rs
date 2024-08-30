use std::{fmt::Debug, str::FromStr};

// Token representing a part of the input expression
#[derive(Debug, Clone, PartialEq)]
pub enum Token<T> {
    Variable(char),
    Constant(T),
    Plus,
    Minus,
    Asterisk,
    Slash,
    OpenParenthesis,
    CloseParenthesis,
    Equal,
}

impl<T> Token<T> {
    pub fn get_variable_char(&self) -> Option<char> {
        if let Token::Variable(c) = self {
            Some(*c)
        } else {
            None
        }
    }
}

// Tokenize the input string
pub fn tokenize<T: std::ops::Add<Output = T> + std::ops::Mul<Output = T> + FromStr + Default>(input: &str) -> Vec<Token<T>>
where
    <T as FromStr>::Err: Debug
{
    let mut tokens = Vec::new();
    let mut iter = input.chars().peekable();

    while let Some(&ch) = iter.peek() {
        match ch {
            'a'..='z' | 'A'..='Z' => {
                tokens.push(Token::Variable(ch));
                iter.next();
            }
            '0'..='9' => {
                let mut value = String::default();

                while let Some(&char) = iter.peek() {
                    if let Some(_) = char.to_digit(10) {
                        value.push(char);
                        iter.next();
                    } else if char == '.' {
                        value.push(char);
                        iter.next();
                    } else {
                        break;
                    }
                }

                tokens.push(Token::Constant(T::from_str(&value).unwrap()));
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
            '/' => {
                tokens.push(Token::Slash);
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
