mod eval;
mod lexer;
mod parser;
mod repl;

use repl::Repl;

fn main() {
    Repl::new().start().unwrap();
}
