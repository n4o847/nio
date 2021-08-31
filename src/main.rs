use clap::{App, Arg};
use nio::codegen::CodeGenerator;
use nio::parser::Parser;
use std::fs::{self, File};
use std::io::{self, Read};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let matches = App::new("nio")
        .version("0.1.0")
        .subcommand(App::new("parse"))
        .subcommand(
            App::new("compile")
                .arg(Arg::new("INPUT").required(true))
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .takes_value(true),
                ),
        )
        .get_matches();

    if let Some(_matches) = matches.subcommand_matches("parse") {
        let mut buffer = String::new();
        let mut stdin = io::stdin();
        stdin.read_to_string(&mut buffer)?;

        let result = Parser::parse(buffer.as_str());
        println!("{:?}", result);
    }

    if let Some(matches) = matches.subcommand_matches("compile") {
        let source = matches.value_of("INPUT").unwrap();
        let target = &match matches.value_of("output") {
            Some(target) => target.to_string(),
            None => source.strip_suffix(".nio").unwrap_or(source).to_string() + ".wasm",
        }[..];

        eprintln!("Compile {}", fs::canonicalize(source)?.display());
        let input = fs::read_to_string(source)?;

        let program = Parser::parse(&input)?;

        let module = CodeGenerator::generate(&program)?;

        eprintln!("Emit {}", fs::canonicalize(target)?.display());
        let mut output = File::create(target)?;
        nio::wasm::binary::emit(&mut output, &module)?;
    }

    Ok(())
}
