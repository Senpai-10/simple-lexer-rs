use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(i32),
    Plus,
    Minus,
    Multiply,
    Divide,
}

pub struct Lexer<'a> {
    chars: Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            chars: input.chars(),
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(char) = self.chars.next() {
            if let Some(token) = self.get_next_token(char) {
                tokens.push(token);
            }
        }

        tokens
    }

    fn get_next_token(&mut self, next_char: char) -> Option<Token> {
        match next_char {
            '*' => Some(Token::Multiply),
            '/' => Some(Token::Divide),
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '0'..='9' => {
                let mut number = next_char.to_digit(10)? as i32;
                while let Some(next_char) = self.chars.clone().next() {
                    if let Some(digit) = next_char.to_digit(10) {
                        number = number * 10 + digit as i32;
                        self.chars.next();
                    } else {
                        break;
                    }
                }
                Some(Token::Number(number))
            }
            _ => None,
        }
    }
}
