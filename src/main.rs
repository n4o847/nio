mod lexer;
mod parser;
mod repl;

fn main() {
    repl::start().unwrap();
}
