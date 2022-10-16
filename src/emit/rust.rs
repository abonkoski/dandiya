use crate::ast::*;

pub const PREAMBLE: &str = "";
pub const POSTAMBLE: &str = "";

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

fn emit_skip(out: &mut dyn std::fmt::Write, skip: &Skip) -> std::fmt::Result {
    for s in &skip.0 {
        match s {
            SkipElem::Whitespace(w) => write!(out, "{}", w)?,
            SkipElem::LineComment(txt) => write!(out, "//{}", txt)?,
        }
    }
    Ok(())
}

fn emit_fn(out: &mut dyn std::fmt::Write, decl: &FuncDecl) -> std::fmt::Result {
    emit_skip(out, &decl.prefix)?;
    write!(
        out,
        "extern \"C\" fn {}_v{}({}){};",
        decl.name,
        decl.version,
        args_str(&decl.args),
        ret_str(&decl.ret)
    )
}

fn emit_struct(out: &mut dyn std::fmt::Write, decl: &StructDecl) -> std::fmt::Result {
    emit_skip(out, &decl.prefix)?;
    write!(out, "#[repr(C)]\n")?;
    write!(out, "struct {} {{\n", decl.name)?;
    for f in &decl.fields {
        write!(out, "  {}: {},\n", f.name, type_str(&f.typ))?;
    }
    write!(out, "}}")
}

fn emit_opaque(out: &mut dyn std::fmt::Write, decl: &OpaqueDecl) -> std::fmt::Result {
    emit_skip(out, &decl.prefix)?;
    write!(out, "#[repr(C)]\n")?;
    write!(out, "struct {} {{_opaque_data: [u8; 0]}}", decl.name)
}

fn emit_const(out: &mut dyn std::fmt::Write, decl: &ConstDecl) -> std::fmt::Result {
    emit_skip(out, &decl.prefix)?;
    write!(out, "const {}: u64 = {};", decl.name, decl.val)
}

pub fn emit(out: &mut dyn std::fmt::Write, defn: &ApiDefn) -> std::fmt::Result {
    write!(out, "{}", PREAMBLE)?;
    for decl in &defn.decls {
        match decl.as_ref() {
            Decl::Fn(decl) => emit_fn(out, decl)?,
            Decl::Struct(decl) => emit_struct(out, decl)?,
            Decl::Opaque(decl) => emit_opaque(out, decl)?,
            Decl::Const(decl) => emit_const(out, decl)?,
        }
    }
    emit_skip(out, &defn.suffix)?;
    write!(out, "{}", POSTAMBLE)?;
    Ok(())
}
