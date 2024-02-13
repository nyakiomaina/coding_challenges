#[derive(PartialEq)]
pub enum Token {
    BraceOpen,
    BraceClose,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    for c in input.chars() {
        match c {
            '{' => tokens.push(Token::BraceOpen),
            '}' => tokens.push(Token::BraceClose),
        }
    }
    tokens
}