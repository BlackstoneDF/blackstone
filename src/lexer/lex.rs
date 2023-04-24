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

                
                if let Some(token) = attempt_tokenize_str(&str_builder) {
                    stream.push(Token {
                        at_line: current_line,
                        at_char: current_char,
                        token
                    });
                    str_builder = String::new();
                }
            }
        }
    }
    vec![]
}

pub fn attempt_tokenize_str(input: &str) -> Option<TokenType> {
    if let Some(ch) = input.chars().nth(0) {
        if let Some(token) = tokenize_single_ch(ch) {
            return Some(token);
        }
    } else if let Some(token) = tokenize_identifier(input) {
        return Some(token);
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

pub fn tokenize_identifier(input: &str) -> Option<TokenType> {
    let valid_starters = ['A' , 'B' , 'C' , 'D' , 'E' , 'F' , 'G', 'H' , 
        'I' , 'J' , 'K' , 'L' , 'M' , 'N', 'O' , 'P' , 
        'Q' , 'R' , 'S' , 'T' , 'U', 'V' , 'W' , 'X' , 
        'Y' , 'Z' , 'a' , 'b', 'c' , 'd' , 'e' , 'f' , 
        'g' , 'h' , 'i', 'j' , 'k' , 'l' , 'm' , 'n' , 
        'o' , 'p', 'q' , 'r' , 's' , 't' , 'u' , 'v' , 
        'w', 'x' , 'y' , 'z', '_'];
    let valid_continuers = ['A' , 'B' , 'C' , 'D' , 'E' , 'F' , 'G', 'H' , 
        'I' , 'J' , 'K' , 'L' , 'M' , 'N', 'O' , 'P' , 
        'Q' , 'R' , 'S' , 'T' , 'U', 'V' , 'W' , 'X' , 
        'Y' , 'Z' , 'a' , 'b', 'c' , 'd' , 'e' , 'f' , 
        'g' , 'h' , 'i', 'j' , 'k' , 'l' , 'm' , 'n' , 
        'o' , 'p', 'q' , 'r' , 's' , 't' , 'u' , 'v' , 
        'w', 'x' , 'y' , 'z' , '_' , '0' , '1' , '2' , 
        '3' , '4' , '5' , '6' , '7' , '8' , '9'];
    if !input.starts_with(valid_starters) {
        return None;
    }
    for ch in input.chars() {
        if !valid_continuers.contains(&ch) {
            return None;
        }    
    }
    return Some(Identifier(input.to_string()));
    
}