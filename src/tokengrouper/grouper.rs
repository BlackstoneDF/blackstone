use crate::lexer::tokens::{Token, TokenType};

#[allow(unused_variables, unused_assignments)]
pub fn transform_to_ast(input: Vec<Token>) -> Vec<Token> {
    let mut output: Vec<Token> = vec![];
    let mut adding_to_block = 0;
    let mut last_adding_to_block = 0;
    let mut adding_to_tuple = 0;
    let mut last_adding_to_tuple = 0;
    for (_count, mut token) in input.into_iter().enumerate() {
        match &token.token {
            TokenType::OpenBraces => {
                token.token = TokenType::Block { tokens: vec![] };
                output.push(token.clone());
                adding_to_block += 1;
            }
            TokenType::CloseBraces => {
                adding_to_block -= 1;
            }
            TokenType::OpenParen => {
                token.token = TokenType::Tuple { tokens: vec![] };
                output.push(token.clone());
                adding_to_tuple += 1;
            }
            TokenType::CloseParen => {
                adding_to_tuple -= 1;
            }
            _ => {
                if adding_to_block == 0 && adding_to_tuple == 0 {
                    println!("will add normally ({adding_to_block}/{adding_to_tuple}), {token:#?}");
                    output.push(token);

                    // do this after
                    last_adding_to_tuple = adding_to_tuple;
                    last_adding_to_block = adding_to_block;
                } else {
                    println!(
                        "will print abnormally ({adding_to_block}/{adding_to_tuple}), {token:#?}"
                    );
                    if adding_to_block > last_adding_to_block {
                        let mut clone = token.clone();
                        clone.token = TokenType::Block { tokens: vec![] };
                        output.push(clone);
                        if let TokenType::Block { ref mut tokens } =
                            &mut output.last_mut().expect("failed to get last somehow").token
                        {
                            tokens.push(token.clone());
                        }
                    } else if adding_to_tuple > last_adding_to_tuple {
                        let mut clone = token.clone();
                        clone.token = TokenType::Tuple { tokens: vec![] };
                        output.push(clone);
                        if let TokenType::Tuple { ref mut tokens } =
                            &mut output.last_mut().expect("failed to get last somehow").token
                        {
                            tokens.push(token.clone());
                        }
                    } else if adding_to_block == last_adding_to_block {
                        if let TokenType::Block { ref mut tokens } =
                            &mut output.last_mut().expect("failed to get last somehow").token
                        {
                            tokens.push(token.clone());
                        }
                    } else if adding_to_tuple == last_adding_to_tuple {
                        if let TokenType::Tuple { ref mut tokens } =
                            &mut output.last_mut().expect("failed to get last somehow").token
                        {
                            tokens.push(token.clone());
                        }
                    }
                    // do this after
                    last_adding_to_tuple = adding_to_tuple;
                    last_adding_to_block = adding_to_block;
                }
            }
        }
    }
    output
}
