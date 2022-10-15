use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct ApiDefn {
    pub symbols: HashMap<String, Rc<Decl>>,
    pub decls: Vec<Rc<Decl>>,
}

#[derive(Debug)]
pub enum Decl {
    Fn(FuncDecl),
    Struct(StructDecl),
}

impl Decl {
    pub fn name(&self) -> &str {
        match self {
            Decl::Fn(decl) => &decl.name,
            Decl::Struct(decl) => &decl.name,
        }
    }
}

#[derive(Debug)]
pub struct FuncDecl {
    pub name: String,
    pub args: Vec<Field>,
    pub ret: ReturnType,
    pub version: usize,
}

#[derive(Debug)]
pub struct StructDecl {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub typ: Type,
}

#[derive(Debug, PartialEq)]
pub enum ReturnType {
    None,
    Some(Type),
}

#[derive(Debug, PartialEq)]
pub enum Type {
    Pointer(Box<Type>),
    Array(Box<Type>, u64),
    Base(BaseType),
}

#[derive(Debug, PartialEq)]
pub enum BaseType {
    Struct(String),
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
}
