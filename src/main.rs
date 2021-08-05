use nio::parser::Parser;
use std::env;
use std::io::{self, Read};
use std::process;

fn main() -> io::Result<()> {
    let args: Vec<_> = env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("parse") => {
            let mut buffer = String::new();
            let mut stdin = io::stdin();
            stdin.read_to_string(&mut buffer)?;

            let result = Parser::parse(buffer.as_str());
            println!("{:?}", result);
            Ok(())
        }
        Some(s) => {
            eprintln!("error: no such subcommand: `{}`", s);
            process::exit(1);
        }
        None => {
            process::exit(0);
        }
    }
}
