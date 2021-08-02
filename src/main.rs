use std::env;
use std::process;

fn main() {
    let args: Vec<_> = env::args().collect();
    match args.get(1).map(String::as_str) {
        Some(s) => {
            eprintln!("error: no such subcommand: `{}`", s);
            process::exit(1);
        }
        None => {
            process::exit(0);
        }
    }
}
