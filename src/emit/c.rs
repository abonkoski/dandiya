use crate::ast::*;

// Returns (front-part, back-part)
fn type_str(t: &Type) -> (String, String) {
    match t {
        Type::Pointer(subtype) => {
            let (front, back) = type_str(subtype);
            (front + "*", back)
        }
        Type::Array(subtype, len) => {
            let (front, back) = type_str(subtype);
            (front, format!("{}[{}]", back, len))
        }
        Type::Base(base) => {
            let s = match base {
                BaseType::Struct(s) => format!("{}_t", s),
                BaseType::U8 => "uint8_t".to_string(),
                BaseType::I8 => "int8_t".to_string(),
                BaseType::U16 => "uint16_t".to_string(),
                BaseType::I16 => "int16_t".to_string(),
                BaseType::U32 => "uint32_t".to_string(),
                BaseType::I32 => "int32_t".to_string(),
                BaseType::U64 => "uint64_t".to_string(),
                BaseType::I64 => "int64_t".to_string(),
            };
            (s, "".to_string())
        }
    }
}

fn field_str(f: &Field) -> String {
    let (front, back) = type_str(&f.typ);
    format!("{} {}{}", front, f.name, back)
}

fn ret_str(t: &ReturnType) -> String {
    match t {
        ReturnType::None => "void".to_string(),
        ReturnType::Some(t) => {
            let (f, b) = type_str(t);
            f + &b
        }
    }
}

fn args_str(args: &[Field]) -> String {
    let mut s = String::new();
    for f in args {
        if !s.is_empty() {
            s += ", ";
        }
        s += &field_str(f);
    }
    s
}

fn emit_fn(out: &mut dyn std::fmt::Write, decl: &FuncDecl) -> std::fmt::Result {
    write!(
        out,
        "{} {}_v{}({});\n",
        ret_str(&decl.ret),
        decl.name,
        decl.version,
        args_str(&decl.args)
    )
}

fn emit_struct(out: &mut dyn std::fmt::Write, decl: &StructDecl) -> std::fmt::Result {
    write!(out, "typedef struct {} {}_t;\n", decl.name, decl.name)?;
    write!(out, "struct {} {{\n", decl.name)?;
    for f in &decl.fields {
        write!(out, "  {};\n", field_str(f))?;
    }
    write!(out, "}};\n")
}

pub fn emit(out: &mut dyn std::fmt::Write, defn: &ApiDefn) -> std::fmt::Result {
    write!(out, "#pragma once\n")?;
    write!(out, "#include <stdint.h>\n")?;
    write!(out, "\n")?;
    for decl in &defn.decls {
        match decl {
            Decl::Fn(decl) => emit_fn(out, decl)?,
            Decl::Struct(decl) => emit_struct(out, decl)?,
        }
    }
    Ok(())
}
