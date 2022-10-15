pub mod ast;
pub mod parse;

pub mod emit_c;
pub mod emit_rust;

#[derive(Debug)]
pub enum Error {
    ParseFailure(String),
    Unknown,
}

pub type Result<Value> = std::result::Result<Value, Error>;
