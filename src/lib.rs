pub mod ast;
pub mod emit;
pub mod parse;

#[derive(Debug)]
pub enum Error {
    ParseFailure(String),
    Unknown,
}

pub type Result<Value> = std::result::Result<Value, Error>;
