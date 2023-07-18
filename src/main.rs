mod lexer;

use lexer::{Lexer, Token};

fn main() {
    let mut lexer = Lexer::new("11 + 7 * 6 - 4 / 2");
    let tokens = lexer.tokenize();

    assert_eq!(
        tokens,
        vec![
            Token::Number(11),
            Token::Plus,
            Token::Number(7),
            Token::Multiply,
            Token::Number(6),
            Token::Minus,
            Token::Number(4),
            Token::Divide,
            Token::Number(2)
        ]
    );
}
