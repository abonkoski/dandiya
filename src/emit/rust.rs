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

fn emit_fn(out: &mut dyn std::fmt::Write, decl: &FuncDecl) -> std::fmt::Result {
    write!(
        out,
        "extern \"C\" fn {}_v{}({}){};\n",
        decl.name,
        decl.version,
        args_str(&decl.args),
        ret_str(&decl.ret)
    )
}

fn emit_struct(out: &mut dyn std::fmt::Write, decl: &StructDecl) -> std::fmt::Result {
    write!(out, "#[repr(C)]\n")?;
    write!(out, "struct {} {{\n", decl.name)?;
    for f in &decl.fields {
        write!(out, "  {}: {},\n", f.name, type_str(&f.typ))?;
    }
    write!(out, "}}\n")
}

fn emit_opaque(out: &mut dyn std::fmt::Write, decl: &OpaqueDecl) -> std::fmt::Result {
    write!(out, "#[repr(C)]\n")?;
    write!(out, "struct {} {{_opaque_data: [u8; 0]}}\n", decl.name)
}

pub fn emit(out: &mut dyn std::fmt::Write, defn: &ApiDefn) -> std::fmt::Result {
    for decl in &defn.decls {
        match decl.as_ref() {
            Decl::Fn(decl) => emit_fn(out, decl)?,
            Decl::Struct(decl) => emit_struct(out, decl)?,
            Decl::Opaque(decl) => emit_opaque(out, decl)?,
        }
    }
    Ok(())
}
