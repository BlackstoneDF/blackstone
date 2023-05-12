use ariadne::*;
use chumsky::Parser;
use codegen::{block::Block, misc::process_block_vec};

use std::{env, io, io::Write, net::TcpStream};

mod codegen;
mod parser;

fn main() -> io::Result<()> {
    let args = env::args().collect::<Vec<_>>();
    env::set_var("RUST_BACKTRACE", "1");

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
            "docs" => todo!(),
            _ => help_message(prefix), //"help" is included in the catch-all
        }
    } else {
        help_message(prefix);
    }

    Ok(())
}

fn compile_with_recode(vector: Vec<Block>, _name: String) {
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

/*

let vector = vector.into_iter().flatten().collect::<Vec<_>>();

            let _ = vector.get(0).expect("codeless?");

            let name = path.to_string();
            println!("\t\x1b[32;1mSending\x1b[0m `{path}` to client.");
            match target {
                CompileTarget::Recode => compile_with_recode(vector, name),
                CompileTarget::Stdout => compile_to_console(vector),
            }
 */
fn process_inputs(input: &str, path: &str, target: CompileTarget) {
    println!("input: {input}");
    let result = parser::parse::parser().parse(input);

    match result.clone().into_result() {
        Ok(vector) => {
            println!("\t\x1b[32;1mSending\x1b[0m `{path}` to client.");
            for subvector in vector {
                let subvector = subvector.into_iter().flatten().collect::<Vec<_>>();
                let _ = subvector.get(0).expect("codeless?");
                let name = path.to_string();

                match target {
                    CompileTarget::Recode => compile_with_recode(subvector, name),
                    CompileTarget::Stdout => compile_to_console(subvector),
                }
            }
        }
        Err(errors) => {
            println!("it's error");
            for e in errors {
                Report::build(ReportKind::Error, (), e.span().start)
                    .with_message(e.reason().to_string())
                    .with_label(Label::new(e.span().start..e.span().end).with_color(Color::Red))
                    .finish()
                    .print(Source::from(input))
                    .expect("failed to print?");
            }
        }
    }
}

fn help_message(prefix: &str) {
    let bold = "\x1b[39;1m";
    let reset = "\x1b[0m";
    println!("{bold}");
    shulker_header();
    println!("{reset}");

    println!(
        r#"
Blackstone's compiler & build tooling

{bold}Usage:{reset} {prefix} [commands]

{bold}Built-in commands:{reset}
    {bold}version{reset}                     Get the current version of Blackstone
    {bold}init{reset}                        Initialize a new Blackstone environment in your current directory
    {bold}build-one [script]{reset}          Builds the code provided & sends it via `recode` mod
    {bold}build{reset}                       Builds all code in the `scripts` directory & sends it via `recode` mod
    {bold}build-stdout [script]{reset}       Sends the code data to the console instead of to `recode`
                                Useful if you don't have `recode` installed
    {bold}build-test{reset}                  Run the tests in the code. (Coming soon!)
    {bold}add [package]{reset}               Add an external package to your scripts
    {bold}recode{reset}                      Gives a link to the `recode` mod, for ease of use with Blackstone
    {bold}help{reset}                        Shows this message
    {bold}docs{reset}                        View documentation (Coming soon!)
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
