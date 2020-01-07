use crate::eval::Evaluator;
use crate::lexer::{Lexer, Token};
use crate::parser::Parser;
use std::io::{self, BufRead, Write};

pub fn start() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    write!(stdout, "> ")?;
    stdout.flush()?;
    for line in stdin.lock().lines() {
        let input = line?;
        let mut l = Lexer::new(&input[..]);
        writeln!(stdout, "Lexer:")?;
        loop {
            let token = l.next_token();
            writeln!(stdout, "  {:?}", token)?;
            match token {
                Token::EOF | Token::Unexpected(_) => break,
                _ => (),
            };
        }
        let mut p = Parser::new(&input[..]);
        let a = p.parse_program();
        writeln!(stdout, "Parser:")?;
        writeln!(stdout, "  {:?}", a)?;
        match a {
            Ok(a) => {
                let e = Evaluator::new();
                let o = e.eval(a);
                writeln!(stdout, "Eval:")?;
                writeln!(stdout, "  {:?}", o)?;
            }
            Err(_) => (),
        }
        write!(stdout, "> ")?;
        stdout.flush()?;
    }
    Ok(())
}
