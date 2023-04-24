mod codegen;
mod lexer;
mod parser;

fn main() {
    help_message();
}

fn help_message() {
    println!(r#"
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
    "#);
}

