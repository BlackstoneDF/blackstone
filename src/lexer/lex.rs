use super::tokens::{Token, TokenType, TokenType::*};

pub fn string_to_tokens(input: String) -> Vec<Token> {
    let mut str_builder = String::new();

    let mut current_char = 0u32;
    let mut current_line = 1u32;

    let mut stream = vec![];
    for ch in input.chars() {
        match ch {
            '\n' => {
                current_char = 0;
                current_line += 1;
            }
            _ => {
                current_char += 1;
                str_builder.push(ch);

                if let Some(token) = attempt_tokenize_str(&str_builder, &current_char, &current_line) {
                    stream.push(token);
                    str_builder = String::new();
                }
            }
        }
    }
    vec![]
}

pub fn attempt_tokenize_str(input: &str, current_char: &u32, current_line: &u32) -> Option<Token> {
    let mut returned_token = Token {
        at_line: *current_line,
        at_char: *current_char,
        token: TokenType::NoType,
    };
    if let Some(ch) = input.chars().nth(0) {
        if let Some(token) = tokenize_single_ch(ch) {
            returned_token.token = token;
            return Some(returned_token);
        }
    } 
    None
}

pub fn tokenize_single_ch(input: char) -> Option<TokenType> {
    match input {
        '(' => Some(OpenParen),
        ')' => Some(CloseParen),
        '{' => Some(OpenBraces),
        '}' => Some(CloseBraces),
        '=' => Some(Equals),
        _ => None
    }
}