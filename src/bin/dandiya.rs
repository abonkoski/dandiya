use clap::{Parser, ValueEnum};
use dandiya::*;

#[derive(Parser, Debug)]
#[command(name = "dandiya")]
#[command(author = "Anthony Bonkoski")]
#[command(
    about = "API generator designed to ensure ABI stability across API changes while supporting multiple languages"
)]
struct Args {
    /// Path to file containing dandiya definition (.dy)
    input: String,

    /// Type of ouput to generate
    #[arg(value_enum, short, long)]
    emit: Emit,
}

#[derive(Debug, Clone, ValueEnum)]
enum Emit {
    Ast,
    CHeader,
    Rust,
}

fn run() -> std::result::Result<(), String> {
    let args = Args::parse();
    let path = &args.input;

    if !path.ends_with(".dy") {
        return Err(format!("Expected a .dy file, found '{}'", path));
    }

    let dat = std::fs::read(path).map_err(|_| format!("Failed to read input file: {}", path))?;

    let txt =
        std::str::from_utf8(&dat).map_err(|_| format!("Input file is not valid utf8: {}", path))?;

    let ast = match parse::parse(txt, Some(path)) {
        Ok(ast) => ast,
        Err(Error::ParseFailure(msg)) => return Err(msg),
        err => panic!("BUG: Unexpected error: {:?}", err),
    };

    match args.emit {
        Emit::Ast => println!("{:#?}", ast),
        Emit::CHeader => print!("{}", emit::emit(&ast, emit::Language::C)),
        Emit::Rust => print!("{}", emit::emit(&ast, emit::Language::Rust)),
    }

    Ok(())
}

fn main() {
    if let Err(errmsg) = run() {
        eprintln!("{}", errmsg);
        std::process::exit(1);
    }
}
