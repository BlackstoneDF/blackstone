use ariadne::*;
#[allow(unused_imports)]
use chumsky::{chain::Chain, Parser};
#[allow(unused_imports)]
use codegen::{block::Block, item::Item, item_data::ItemData, misc::process_block_vec};

use std::{env, io::Write, net::TcpStream};

mod codegen;
mod parser;

fn main() -> std::io::Result<()> {
    let args = env::args().collect::<Vec<_>>();

    let _input = "";

    let start = std::time::Instant::now();

    if let Some(arg) = args.get(1) {
        // command line commands
        if arg.contains("build-all") {
            println!("\t\x1b[32;1mBuilding\x1b[0m from `./scripts` directory.");
            let paths = std::fs::read_dir("./scripts")?;
            let mut handles = vec![];
            for path in paths {
                handles.push(std::thread::spawn(move || {
                    let display = path
                        .expect("somehow doesnt exist")
                        .path()
                        .display()
                        .to_string();
                    println!("\t\x1b[32;1mBuilding\x1b[0m `{display}`.");
                    let file = std::fs::read_to_string(display.clone()).expect("somehow doesnt exist");
                    process_inputs(&file, &display);
                }));
            }
            for handle in handles {
                handle.join().expect("failed to join");
            }

            let dur = start.elapsed();
            println!("time taken: {}ms", dur.as_millis());
        } else if arg.contains("build") {
            if let Some(arg2) = args.get(1) {
                let file = std::fs::read_to_string(arg2)?;
                process_inputs(&file, &arg2);
            }
        } else if arg.contains("version") {
            //find the toml file - that has the version
            //3rd line has version as `version = "[version]"
            let cargo_toml = include_str!("../Cargo.toml");
            match cargo_toml.lines().nth(2) {
                Some(line) => {
                    let line = line.strip_prefix("version = ").unwrap_or("");
                    if line.is_empty() {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "Did not find the correct version line.",
                        ));
                    }
                    let vers = line.trim_matches('"');
                    println!("Current version: {vers}")
                }
                None => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Could not find a valid `Cargo.toml` file",
                    ))
                }
            }
        }  else if arg.contains("help") { help_message(); } else {
            help_message()
        }
    } else {
        help_message();
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

fn process_inputs(input: &str, path: &str) {
    match parser::parse::parser().parse(input) {
        Ok(vector) => {
            let vector = vector
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            
            println!("\t\x1b[32;1mSending\x1b[0m `{path}` to client.");
            compile_with_recode(vector);
        }
        Err(v) => {
            for err in v {
                Report::build(ReportKind::Error, (), err.span().start)
                    .with_message(format!("{:#?}", err.reason()))
                    .with_label(Label::new(err.span()).with_message(format!(
                        "expected '{}', found '{}'",
                        err.expected().next().unwrap_or(&Some('?')).unwrap_or('!'),
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
    shulker_header();
    println!(
        r#"
Blackstone's compiler & build tooling

Usage: shulker [commands]

Built-in commands:
    version                 Get the current version of Blackstone.
    init                    Initialize a new Blackstone environment in your current directory.
    build [script]          Builds the code provided & sends it via `recode` mod.
    build-all               Builds all code in the `scripts` directory & sends it via `recode` mod.
    build-stdout [script]   Sends the code data to the console instead of to `recode`.
                            Useful if you don't have `recode` installed.
    build-test              Run the tests in the code.
    add [package]           Add an external package to your scripts.
    help                    Shows this message!
    "#
    );
}

fn shulker_header() {
    println!(r#"
    .dP"Y8 88  88 88   88 88     88  dP 888888 88""Yb 
    `Ybo." 88  88 88   88 88     88odP  88__   88__dP 
    o.`Y8b 888888 Y8   8P 88  .o 88"Yb  88""   88"Yb  
    8bodP' 88  88 `YbodP' 88ood8 88  Yb 888888 88  Yb 
    "#);
}

#[allow(dead_code)]
fn blackstone_header() {
    println!(r#"
    88""Yb 88        db     dP""b8 88  dP .dP"Y8 888888  dP"Yb  88b 88 888888 
    88__dP 88       dPYb   dP   `" 88odP  `Ybo."   88   dP   Yb 88Yb88 88__   
    88""Yb 88  .o  dP__Yb  Yb      88"Yb  o.`Y8b   88   Yb   dP 88 Y88 88""   
    88oodP 88ood8 dP""""Yb  YboodP 88  Yb 8bodP'   88    YbodP  88  Y8 888888 
    "#);
}