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
    C,
    Rust,
}

fn run() -> i32 {
    let args = Args::parse();
    let path = &args.input;

    if !path.ends_with(".dy") {
        eprintln!("Expected a .dy file, found '{}'", path);
        return 1;
    }

    let dat = match std::fs::read(path) {
        Ok(dat) => dat,
        Err(_) => {
            eprintln!("Failed to read input file: {}", path);
            return 1;
        }
    };

    let s = match std::str::from_utf8(&dat) {
        Ok(s) => s,
        Err(_) => {
            eprintln!("Input file is not valid utf8: {}", path);
            return 1;
        }
    };

    let ast = match parse::parse(s, Some(path)) {
        Ok(ast) => ast,
        Err(Error::ParseFailure(msg)) => {
            eprint!("{}", msg);
            return 1;
        }
        err => panic!("BUG: Unexpected error: {:?}", err),
    };

    match args.emit {
        Emit::Ast => println!("{:#?}", ast),
        Emit::C => print!("{}", emit::emit(&ast, emit::Language::C)),
        Emit::Rust => print!("{}", emit::emit(&ast, emit::Language::Rust)),
    }

    0
}

fn main() {
    std::process::exit(run());
}
