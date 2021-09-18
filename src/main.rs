use clap::{App, AppSettings, Arg};
use nio::{codegen::CodeGenerator, parser, typecheck};
use std::{
    fs::{self, File},
    io::{self, Read},
    process,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let matches = App::new("nio")
        .version(env!("CARGO_PKG_VERSION"))
        .setting(AppSettings::SubcommandRequiredElseHelp)
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

    match matches.subcommand() {
        Some(("parse", _)) => {
            let mut buffer = String::new();
            let mut stdin = io::stdin();
            stdin.read_to_string(&mut buffer)?;

            let result = parser::parse(buffer.as_str());
            println!("{:?}", result);
        }

        Some(("compile", matches)) => {
            let source = matches.value_of("INPUT").unwrap();
            let target = &match matches.value_of("output") {
                Some(target) => target.to_string(),
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
            nio::wasm::binary::emit(&mut output, &module)?;
        }

        _ => {}
    }

    Ok(())
}
