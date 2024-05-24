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
}

impl Token {
    pub fn is_variable(&self) -> bool {
        matches!(self, Token::Variable(_))
    }

    pub fn get_variable_char(&self) -> Option<char> {
        if let Token::Variable(c) = self {
            Some(*c)
        } else {
            None
        }
    }
}
