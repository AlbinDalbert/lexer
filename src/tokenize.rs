use crate::token::Token;

mod Token;

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut word_buffer = String::new();
    let mut number_buffer = String::new();

    for c in input.chars() {
        // check for word
        if c.is_alphabetic() {
            word_buffer.push(c);
        } else if !word_buffer.is_empty() {
            tokens.push(Token::Word(word_buffer.clone()));
            word_buffer.clear();
        }

        if c.is_numeric() && word_buffer.is_empty() {
            number_buffer.push(c);
        } else if !number_buffer.is_empty() {
            tokens.push(Token::Number(number_buffer.parse()?));
            number_buffer.clear();
        }
    }
}
