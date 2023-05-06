use ariadne::*;
use chumsky::error::SimpleReason;
use chumsky::Parser;
use codegen::{block::Block, misc::process_block_vec};

use std::{env, io, io::Write, net::TcpStream};

mod codegen;
mod parser;

fn main() -> io::Result<()> {
    let args = env::args().collect::<Vec<_>>();

    let start = std::time::Instant::now();

    let prefix = "shulker"; // cmd prefix

    if let Some(arg) = args.get(1) {
        match arg.as_str() {
            "build" => {
                println!("\t\x1b[32;1mBuilding\x1b[0m from `./scripts` directory.");
                let paths = std::fs::read_dir("./scripts")?;
                for path in paths {
                    let display = path
                        .expect("somehow doesnt exist")
                        .path()
                        .display()
                        .to_string();
                    println!("\t\x1b[32;1mBuilding\x1b[0m `{display}`.");
                    let file =
                        std::fs::read_to_string(display.clone()).expect("somehow doesnt exist");
                    process_inputs(&file, &display, CompileTarget::Recode);
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }

                let dur = start.elapsed();
                println!("time taken: {}ms", dur.as_millis());
            }
            "build-one" => {
                if let Some(arg2) = args.get(1) {
                    let file = std::fs::read_to_string(arg2)?;
                    process_inputs(&file, arg2, CompileTarget::Recode);
                } else {
                    println!("There needs to be a string for a file path after the command,\ne.g `{prefix} build-one /foo/bar.bls`\n");
                }
            }
            "build-stdout" => {
                if let Some(arg2) = args.get(1) {
                    let file = std::fs::read_to_string(arg2)?;
                    process_inputs(&file, arg2, CompileTarget::Stdout);
                } else {
                    println!("There needs to be a string for a file path after the command,\ne.g `{prefix} build-stdout /foo/bar.bls`\n");
                }
            }
            "version" => {
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
            }
            "recode" => {
                println!("Recode on modrinth: https://modrinth.com/mod/recode");
                println!("Recode on github: https://github.com/homchom/recode");
            }
            _ => help_message(prefix), //"help" is included in the catch-all
        }
    } else {
        help_message(prefix);
    }

    Ok(())
}

fn compile_with_recode(vector: Vec<Block>, name: String) {
    let s = process_block_vec(vector);
    let send =
        r#"{"type": "template","source": "Blackstone","data":"{'name':'my name','data':'%s%'}"}"#;
    let send = send.replace("%s%", &s);
    // let send = send.replace("%n%", &name);
    let mut stream = TcpStream::connect("localhost:31372").expect("failed to connect");
    stream
        .write_all(send.as_bytes())
        .expect("failed to write all");
}

fn compile_to_console(vector: Vec<Block>) {
    println!("{}", process_block_vec(vector));
    println!("--------------");
    println!("Paste the above into DF to get it as a template.");
}

fn process_inputs(input: &str, path: &str, target: CompileTarget) {
    match parser::parse::parser().parse(input) {
        Ok(vector) => {
            let vector = vector.into_iter().flatten().collect::<Vec<_>>();

            let _ = vector.get(0).expect("codeless?");

            let name = path.to_string();
            println!("\t\x1b[32;1mSending\x1b[0m `{path}` to client.");
            match target {
                CompileTarget::Recode => compile_with_recode(vector, name),
                CompileTarget::Stdout => compile_to_console(vector),
            }
        }
        Err(v) => {
            for err in v {
                if let SimpleReason::Unexpected = err.reason() {
                    Report::build(ReportKind::Error, (), err.span().start)
                        .with_message("Unexpected tokens")
                        .with_label(Label::new(err.span()).with_message({
                            let mut out = String::new();
                            for expected in err.expected() {
                                out.push_str(format!("'{}' |", expected.unwrap_or('!')).as_str());
                            }
                            out.pop();
                            out.pop();
                            format!("expected {}, found '{}'", out, err.found().unwrap_or(&'✗'))
                        }))
                        .finish()
                        .print(Source::from(input))
                        .unwrap();
                }
                if let SimpleReason::Custom(msg) = err.reason() {
                    Report::build(ReportKind::Error, (), err.span().start)
                        .with_message(msg)
                        .with_label(Label::new(err.span()).with_message("Error occured here"))
                        .finish()
                        .print(Source::from(input))
                        .unwrap();
                }
            }
        }
    }
}

fn help_message(prefix: &str) {
    shulker_header();
    println!(
        r#"
Blackstone's compiler & build tooling

Usage: {prefix} [commands]

Built-in commands:
    version                     Get the current version of Blackstone.
    init                        Initialize a new Blackstone environment in your current directory.
    build-one [script]          Builds the code provided & sends it via `recode` mod.
    build                       Builds all code in the `scripts` directory & sends it via `recode` mod.
    build-stdout [script]       Sends the code data to the console instead of to `recode`.
                                Useful if you don't have `recode` installed.
    build-test                  Run the tests in the code.
    add [package]               Add an external package to your scripts.
    recode                      Gives a link to the `recode` mod, for ease of use with Blackstone
    help                        Shows this message!
    "#
    );
}

fn shulker_header() {
    println!(
        r#"
    .dP"Y8 88  88 88   88 88     88  dP 888888 88""Yb 
    `Ybo." 88  88 88   88 88     88odP  88__   88__dP 
    o.`Y8b 888888 Y8   8P 88  .o 88"Yb  88""   88"Yb  
    8bodP' 88  88 `YbodP' 88ood8 88  Yb 888888 88  Yb 
    "#
    );
}

#[allow(dead_code)]
fn blackstone_header() {
    println!(
        r#"
    88""Yb 88        db     dP""b8 88  dP .dP"Y8 888888  dP"Yb  88b 88 888888 
    88__dP 88       dPYb   dP   `" 88odP  `Ybo."   88   dP   Yb 88Yb88 88__   
    88""Yb 88  .o  dP__Yb  Yb      88"Yb  o.`Y8b   88   Yb   dP 88 Y88 88""   
    88oodP 88ood8 dP""""Yb  YboodP 88  Yb 8bodP'   88    YbodP  88  Y8 888888 
    "#
    );
}

enum CompileTarget {
    Recode,
    Stdout,
}
