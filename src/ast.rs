use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct ApiDefn {
    pub symbols: HashMap<String, Rc<Decl>>,
    pub apis: Apis,
    pub decls: Vec<Rc<Decl>>,
    pub suffix: Skip,
}

#[derive(Debug)]
pub struct Api {
    pub name: String,
    pub latest: Version,
    pub all_versions: HashMap<Version, Rc<Decl>>,
}

impl Api {
    pub fn latest(&self) -> &Decl {
        self.all_versions.get(&self.latest).unwrap().as_ref()
    }
}

#[derive(Debug)]
pub struct Apis {
    pub name_to_api_idx: HashMap<String, usize>,
    pub apis: Vec<Api>,
}

impl Apis {
    pub fn new() -> Self {
        Self {
            name_to_api_idx: HashMap::new(),
            apis: Vec::new(),
        }
    }

    pub fn latest(&self, name: &str) -> Option<&Decl> {
        let idx = self.name_to_api_idx.get(name)?;
        Some(self.apis[*idx].latest())
    }

    pub fn insert(&mut self, name: String, version: Version, decl: Rc<Decl>) -> Option<Rc<Decl>> {
        if let Some(idx) = self.name_to_api_idx.get(&name) {
            // already exists: add next version
            let api = &mut self.apis[*idx];
            if let Some(old) = api.all_versions.insert(version, decl) {
                // version already exists
                return Some(old);
            }
            if version.0 > api.latest.0 {
                api.latest = version;
            }
        } else {
            // first version of a new api
            let mut all_versions = HashMap::new();
            all_versions.insert(version, decl);
            self.apis.push(Api {
                name: name.clone(),
                latest: version,
                all_versions,
            });
            self.name_to_api_idx.insert(name, self.apis.len() - 1);
        }
        None
    }
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
            Decl::Fn(decl) => format!("{}_v{}", decl.name, decl.version.0),
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
    pub version: Version,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Version(pub u64);

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
