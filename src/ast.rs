#[derive(Debug)]
pub struct ApiDefn {
    pub decls: Vec<Decl>,
}

#[derive(Debug)]
pub enum Decl {
    Fn(FuncDecl),
    Struct(StructDecl),
}

#[derive(Debug)]
pub struct FuncDecl {
    pub name: String,
    pub args: Vec<Field>,
    pub ret: Type,
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

#[derive(Debug)]
pub enum Type {
    None,
    Pointer(BaseType),
    Value(BaseType),
}

#[derive(Debug)]
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
