use dandiya::*;

fn main() {
    std::process::exit(run());
}

fn run() -> i32 {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: {} <defn.dy>", args[0]);
        return 1;
    }

    let path = &args[1];
    if !path.ends_with(".dy") {
        eprintln!("expected a .dy file, found '{}'", path);
        return 1;
    }

    let dat = std::fs::read(path).unwrap(); // FIXME
    let s = std::str::from_utf8(&dat).unwrap();

    let ast = match parse::parse(s, Some(path)) {
        Ok(ast) => ast,
        Err(Error::ParseFailure(msg)) => {
            eprint!("{}", msg);
            return 1;
        }
        err => panic!("Unexpected error: {:?}", err),
    };

    println!("==========================================================");
    println!(" AST");
    println!("==========================================================");
    println!("{:#?}", ast);

    println!("==========================================================");
    println!(" C Codegen");
    println!("==========================================================");
    emit::emit(&ast, emit::Language::C);

    println!("==========================================================");
    println!(" Rust Codegen");
    println!("==========================================================");
    emit::emit(&ast, emit::Language::Rust);

    0
}
