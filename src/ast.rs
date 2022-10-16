use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct ApiDefn {
    pub symbols: HashMap<String, Rc<Decl>>,
    pub decls: Vec<Rc<Decl>>,
    pub suffix: Skip,
}

#[derive(Debug, Clone)]
pub struct Skip(pub Vec<SkipElem>);

#[derive(Debug, Clone)]
pub enum SkipElem {
    Whitespace(String),
    LineComment(String),
    BlockComment(String),
}

#[derive(Debug)]
pub enum Decl {
    Fn(FuncDecl),
    Struct(StructDecl),
    Opaque(OpaqueDecl),
    Const(ConstDecl),
}

impl Decl {
    pub fn name(&self) -> String {
        match self {
            Decl::Fn(decl) => format!("{}_v{}", decl.name, decl.version),
            Decl::Struct(decl) => decl.name.clone(),
            Decl::Opaque(decl) => decl.name.clone(),
            Decl::Const(decl) => decl.name.clone(),
        }
    }
}

#[derive(Debug)]
pub struct FuncDecl {
    pub prefix: Skip,
    pub name: String,
    pub args: Vec<Field>,
    pub ret: ReturnType,
    pub version: usize,
}

#[derive(Debug)]
pub struct StructDecl {
    pub prefix: Skip,
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Debug)]
pub struct OpaqueDecl {
    pub prefix: Skip,
    pub name: String,
}

#[derive(Debug)]
pub struct ConstDecl {
    pub prefix: Skip,
    pub name: String,
    pub val: u64,
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
