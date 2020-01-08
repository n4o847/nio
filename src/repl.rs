use crate::eval::Evaluator;
use crate::lexer::{Lexer, Token};
use crate::parser::Parser;
use std::io::{self, BufRead, Write};

pub struct Repl {
    lexer: bool,
    parser: bool,
}

impl Repl {
    pub fn new() -> Repl {
        Repl {
            lexer: true,
            parser: true,
        }
    }

    pub fn start(&self) -> io::Result<()> {
        let stdin = io::stdin();
        let mut stdout = io::stdout();
        write!(stdout, "> ")?;
        stdout.flush()?;
        for line in stdin.lock().lines() {
            let input = line?;
            if self.lexer {
                let mut l = Lexer::new(&input[..]);
                writeln!(stdout, "Lexer:")?;
                write!(stdout, "  ")?;
                loop {
                    let token = l.next_token();
                    write!(stdout, "{:?}", token)?;
                    match token {
                        Token::EOF | Token::Unexpected(_) => break,
                        _ => {}
                    };
                    write!(stdout, ", ")?;
                }
                writeln!(stdout)?;
            }
            let mut p = Parser::new(&input[..]);
            let a = p.parse_program();
            if self.parser {
                writeln!(stdout, "Parser:")?;
                writeln!(stdout, "  {:?}", a)?;
            }
            match a {
                Ok(a) => {
                    let mut e = Evaluator::new();
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
}
