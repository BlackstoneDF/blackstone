use ariadne::*;
use chumsky::Parser;
use codegen::{block::Block, item::Item, item_data::ItemData, misc::process_block_vec};

use std::{env, io::Write, net::TcpStream};

mod codegen;
mod parser;

fn main() -> std::io::Result<()> {
    help_message();

    let args = env::args().collect::<Vec<_>>();

    let input = "";

    let start = std::time::Instant::now();

    if let Some(arg) = args.get(1) {
        if arg.contains("build-all") {
            let paths = std::fs::read_dir("./scripts")?;

            for path in paths {
                let handle = std::thread::spawn(move || {
                    let display = path
                        .expect("somehow doesnt exist")
                        .path()
                        .display()
                        .to_string();
                    let file = std::fs::read_to_string(display).expect("somehow doesnt exist");
                    process_inputs(&file);
                });
                handle.join().expect("failed to start thread");
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
        } else if arg.contains("build") {
            if let Some(arg2) = args.get(1) {
                let file = std::fs::read_to_string(arg2)?;
                process_inputs(&file);
            }
        }
    } else {
        help_message();
        return Ok(());
    }

    Ok(())
}

fn compile_with_recode(vector: Vec<Block>) {
    let s = process_block_vec(vector);
    let send =
        r#"{"type": "template","source": "Blackstone","data":"{'name':'Test','data':'%s%'}"}"#;
    let send = send.replace("%s%", &s);
    let mut stream = TcpStream::connect("localhost:31372").expect("failed to connect");
    stream
        .write_all(send.as_bytes())
        .expect("failed to write all");
}
#[allow(dead_code, unused_variables, unused_mut)]
fn compile_with_codeclient(vector: Vec<Block>) {
    let mut stream = TcpStream::connect("localhost:31375").expect("failed to connect");
    println!("Readying! Please type `/auth` ingame to continue.");
}

fn process_inputs(input: &str) {
    match parser::parse::parser().parse(input) {
        Ok(vector) => {
            let vector = vector
                .into_iter()
                .filter(|f| f.is_some())
                .map(|f| f.expect("impossible to fail"))
                .collect::<Vec<_>>();

            compile_with_recode(vector);
        }
        Err(v) => {
            for err in v {
                Report::build(ReportKind::Error, (), err.span().start)
                    .with_message(format!("{:#?}", err.reason()))
                    .with_label(Label::new(err.span()).with_message(format!(
                        "expected '{}', found '{}'",
                        err.expected().nth(0).unwrap_or(&Some('?')).unwrap_or('!'),
                        err.found().unwrap_or(&'✗')
                    )))
                    .finish()
                    .print(Source::from(input))
                    .unwrap();
            }
        }
    }
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
