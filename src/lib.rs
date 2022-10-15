mod ast;

mod parse;
pub use parse::*;

pub mod emit_c;

#[derive(Debug)]
pub enum Error {
    Unknown,
}

pub type Result<Value> = std::result::Result<Value, Error>;
