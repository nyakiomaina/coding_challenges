use crate::lexer::Token;

pub fn parse(tokens: Vec<Token>) -> bool {
    tokens == vec![Token::BraceOpen, Token::BraceClose]
}