use crate::ast::*;

fn type_str(t: &Type) -> String {
    let (base, is_ptr) = match t {
        Type::None => return "()".to_string(),
        Type::Pointer(base) => (base, true),
        Type::Value(base) => (base, false),
    };

    let base_str = match base {
        BaseType::Struct(s) => s,
        BaseType::U8 => "u8",
        BaseType::I8 => "i8",
        BaseType::U16 => "u16",
        BaseType::I16 => "i16",
        BaseType::U32 => "u32",
        BaseType::I32 => "i32",
        BaseType::U64 => "u64",
        BaseType::I64 => "i64",
    };

    let mut s = String::new();
    if is_ptr {
        s += "*mut ";
    }
    s += base_str;

    s
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

fn ret_str(ret: &Type) -> String {
    if ret == &Type::None {
        return "".to_string();
    }
    format!(" -> {}", &type_str(ret))
}

fn emit_fn(decl: &FuncDecl) {
    println!(
        "fn {}({}){}",
        decl.name,
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
