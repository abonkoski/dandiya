use hydra::*;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: {} <defn-file>", args[0]);
        std::process::exit(1);
    }
    let path = &args[1];

    let dat = std::fs::read(path).unwrap(); // FIXME
    let s = std::str::from_utf8(&dat).unwrap();

    let ast = Parser::new(s, Some(path)).parse();

    println!("==========================================================");
    println!(" AST");
    println!("==========================================================");
    println!("{:#?}", ast);

    println!("==========================================================");
    println!(" C Codegen");
    println!("==========================================================");
    emit_c::emit(&ast);

    println!("==========================================================");
    println!(" Rust Codegen");
    println!("==========================================================");
    emit_rust::emit(&ast);
}
