use clap::{Parser, Subcommand};
use nio::{codegen::CodeGenerator, parser, typecheck};
use std::{
    fs::{self, File},
    io::{self, Read},
    process,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Parser)]
#[clap(version)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Parse,

    Compile {
        /// Input file
        input: String,

        /// Output file
        #[clap(short, long)]
        output: Option<String>,
    },
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Parse => {
            let mut buffer = String::new();
            let mut stdin = io::stdin();
            stdin.read_to_string(&mut buffer)?;

            let result = parser::parse(buffer.as_str());
            println!("{:?}", result);
        }

        Command::Compile { input, output } => {
            let source = input.as_str();
            let target = &match output {
                Some(target) => target,
                None => source.strip_suffix(".nio").unwrap_or(source).to_string() + ".wasm",
            }[..];

            eprintln!("Compile {}", fs::canonicalize(source)?.display());
            let input = fs::read_to_string(source)?;

            let program = parser::parse(&input).unwrap_or_else(|err| {
                eprintln!("ParseError: {}", err);
                process::exit(1);
            });

            let mut program = program.into();

            typecheck::typecheck(&mut program).unwrap_or_else(|err| {
                eprintln!("TypeError: {}", err);
                process::exit(1);
            });

            let module = CodeGenerator::generate(&program)?;

            let mut output = File::create(target)?;
            eprintln!("Emit {}", fs::canonicalize(target)?.display());
            nio::wasm::emit(&mut output, &module)?;
        }
    }

    Ok(())
}
