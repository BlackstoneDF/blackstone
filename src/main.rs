mod codegen;

fn main() {
    help_message();
}

fn help_message() {
    println!(r#"
Blackstone's compiler & build tooling

Usage: blackstone [commands]

Built-in commands:
    version                 Get the current version of DFScript.
    init                    Initialize a new DFScript environment in your current directory.
    build [script]          Builds the script provided & sends it via `recode` mod.
    build-all               Builds all scripts in the `scripts` directory & sends it via `recode` mod.
    build-stdout [script]   Sends the script data to the console instead of to `recode`.
                            Useful if you don't have `recode` installed.
    build_test              Run the build test.
    add [package]           Add an external package to your scripts.
    "#);
}