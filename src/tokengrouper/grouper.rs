use crate::lexer::tokens::{Token, TokenType};

pub fn transform_to_ast(input: Vec<Token>) -> Vec<Token> {
    let mut output: Vec<Token> = vec![];
    let mut adding_to_block = 0;
    let mut adding_to_tuple = 0;
    for (count, mut token) in input.into_iter().enumerate() {
        match &token.token {
            TokenType::OpenBraces => {
                token.token = TokenType::Block(vec![]);
                output.push(token.clone());
                adding_to_block += 1;
            }
            TokenType::CloseBraces => {
                output.push(token.clone());
                adding_to_block -= 1;
            }
            TokenType::OpenParen => {
                token.token = TokenType::Tuple(vec![]);
                output.push(token.clone());
                adding_to_tuple += 1;
            }
            TokenType::CloseParen => {
                output.push(token.clone());
                adding_to_tuple -= 1;
            }
            _ => {
                if adding_to_block == 0 && adding_to_tuple == 0 {
                    println!("will add normally ({adding_to_block}/{adding_to_tuple}), {:#?}", token);
                    output.push(token);
                } else {
                    println!("will print abnormally ({adding_to_block}/{adding_to_tuple}), {:#?}", token);
                }
            }
        }
        
    }
    output
}
