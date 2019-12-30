mod lexer;
mod parser;
mod eval;
mod repl;

fn main() {
    repl::start().unwrap();
}
