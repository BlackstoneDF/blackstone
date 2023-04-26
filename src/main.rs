use codegen::{block::Block, item::Item, item_data::ItemData, misc::process_block_vec};

use crate::{
    lexer::{
        lex::Lexer,
        tokens::{Token, TokenType},
    },
    tokengrouper::grouper::transform_to_ast,
};

mod codegen;
mod ir;
mod lexer;
mod tokengrouper;

fn main() {
    help_message();

    let _s = process_block_vec(vec![
        Block::EventDefinition {
            block: "event",
            action: "Join",
        },
        Block::Code {
            block: "player_action",
            items: vec![Item {
                id: "txt".to_string(),
                slot: 0,
                item: ItemData::Text {
                    data: "Hello world!".to_string(),
                },
            }],
            action: "SendMessage",
            data: "",
        },
    ]);
    // println!("{s}");
    /* let send =
        r#"{"type": "template","source": "Blackstone","data":"{'name':'Test','data':'%s%'}"}"#;
    let send = send.replace("%s%", &s);
    println!("{send}");

    let mut stream = TcpStream::connect("localhost:31372").expect("failed to connect");
    stream
        .write_all(send.as_bytes())
        .expect("failed to write all"); */

    let input = r#"
    playerEvent.join()
    {
        player.sendMessage("Hello world!");
    }"#;
    let mut lexer = Lexer::new(input.to_string());
    let mut c = 0;
    let mut tokens = vec![];
    loop {
        c += 1;
        let tok = lexer.read_token();
        let _line = 0;
        let at_char = lexer.position;
        tokens.push(Token {
            at_char: at_char as u32,
            at_line: 0,
            token: tok.clone(),
        });
        if let TokenType::Eof = tok {
            break;
        }
        if c > 100 {
            break;
        }
    }
    // println!("tokens: {tokens:#?}");
    let ast = transform_to_ast(tokens);

    println!("{ast:#?}");
    // println!("{ast:#?}");
}

fn help_message() {
    println!(
        r#"
Blackstone's compiler & build tooling

Usage: bls [commands]

Built-in commands:
    version                 Get the current version of Blackstone.
    init                    Initialize a new Blackstone environment in your current directory.
    build [script]          Builds the code provided & sends it via `recode` mod.
    build-all               Builds all code in the `scripts` directory & sends it via `recode` mod.
    build-stdout [script]   Sends the code data to the console instead of to `recode`.
                            Useful if you don't have `recode` installed.
    build_test              Run the tests in the code.
    add [package]           Add an external package to your scripts.
    "#
    );
}
