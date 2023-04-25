use crate::lexer::tokens::{Token, TokenType};

pub fn transform_to_ast(input: Vec<Token>) -> Vec<Token> {
    let mut output: Vec<Token> = vec![];
    let mut adding_to_block = 0;
    let mut last_adding_to_block = 0;
    let mut adding_to_tuple = 0;
    let mut last_adding_to_tuple = 0;
    for (count, mut token) in input.into_iter().enumerate() {
        match &token.token {
            TokenType::OpenBraces => {
                token.token = TokenType::Block(vec![]);
                output.push(token.clone());
                adding_to_block += 1;
            }
            TokenType::CloseBraces => {
                adding_to_block -= 1;
            }
            TokenType::OpenParen => {
                token.token = TokenType::Tuple(vec![]);
                output.push(token.clone());
                adding_to_tuple += 1;
            }
            TokenType::CloseParen => {
                adding_to_tuple -= 1;
            }
            _ => {
                if adding_to_block == 0 && adding_to_tuple == 0 {
                    println!(
                        "will add normally ({adding_to_block}/{adding_to_tuple}), {:#?}",
                        token
                    );
                    output.push(token);

                    // do this after
                    last_adding_to_tuple = adding_to_tuple;
                    last_adding_to_block = adding_to_block;
                } else {
                    println!(
                        "will print abnormally ({adding_to_block}/{adding_to_tuple}), {:#?}",
                        token
                    );

                    // do this after
                    last_adding_to_tuple = adding_to_tuple;
                    last_adding_to_block = adding_to_block;
                }
            }
        }
    }
    output
}
