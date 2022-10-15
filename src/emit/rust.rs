use crate::ast::*;

fn type_str(t: &Type) -> String {
    match t {
        Type::Pointer(subtype) => format!("*mut {}", type_str(subtype)),
        Type::Array(subtype, len) => {
            format!("[{}; {}]", type_str(subtype), len)
        }
        Type::Base(base) => match base {
            BaseType::Struct(s) => s,
            BaseType::U8 => "u8",
            BaseType::I8 => "i8",
            BaseType::U16 => "u16",
            BaseType::I16 => "i16",
            BaseType::U32 => "u32",
            BaseType::I32 => "i32",
            BaseType::U64 => "u64",
            BaseType::I64 => "i64",
        }
        .to_string(),
    }
}

fn ret_str(t: &ReturnType) -> String {
    match t {
        ReturnType::None => "".to_string(),
        ReturnType::Some(t) => format!(" -> {}", type_str(t)),
    }
}

fn args_str(args: &[Field]) -> String {
    let mut s = String::new();
    for f in args {
        if !s.is_empty() {
            s += ", ";
        }
        s += &format!("{}: {}", f.name, type_str(&f.typ));
    }
    s
}

fn emit_fn(decl: &FuncDecl) {
    println!(
        "fn {}_v{}({}){}",
        decl.name,
        decl.version,
        args_str(&decl.args),
        ret_str(&decl.ret)
    );
}

fn emit_struct(decl: &StructDecl) {
    println!("struct {} {{", decl.name);
    for f in &decl.fields {
        println!("    {}: {},", f.name, type_str(&f.typ));
    }
    println!("}}");
}

pub fn emit(defn: &ApiDefn) {
    for decl in &defn.decls {
        match decl {
            Decl::Fn(decl) => emit_fn(decl),
            Decl::Struct(decl) => emit_struct(decl),
        }
    }
}
