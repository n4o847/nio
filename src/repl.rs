use crate::lexer::{Lexer, Token};
use crate::parser::Parser;
use std::io::{self, BufRead, Write};

pub fn start() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    for line in stdin.lock().lines() {
        let input = line?;
        let mut l = Lexer::new(&input[..]);
        loop {
            let token = l.next_token();
            writeln!(stdout, "{:?}", token)?;
            match token {
                Token::EOF | Token::Unexpected(_) => break,
                _ => (),
            };
        }
        let mut p = Parser::new(&input[..]);
        writeln!(stdout, "{:?}", p.parse_program())?;
        stdout.flush()?;
    }
    Ok(())
}
