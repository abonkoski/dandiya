use crate::ast::*;

pub const PREAMBLE: &str = "\
#pragma once
#include <stdint.h>

#ifdef __cplusplus
extern \"C\" {
#endif
";

pub const POSTAMBLE: &str = "
#ifdef __cplusplus
}
#endif
";

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
    // quirky C: empty args is (void)
    if s.is_empty() {
        s = "void".to_string();
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
    write!(out, "struct {} {{\n", decl.name)?;
    for f in &decl.fields {
        write!(out, "  {};\n", field_str(f))?;
    }
    write!(out, "}};\n")?;
    write!(out, "\n")
}

fn emit_typedef(out: &mut dyn std::fmt::Write, name: &str) -> std::fmt::Result {
    write!(out, "typedef struct {} {}_t;\n", name, name)
}

pub fn emit(out: &mut dyn std::fmt::Write, defn: &ApiDefn) -> std::fmt::Result {
    write!(out, "{}", PREAMBLE)?;
    write!(out, "\n")?;

    // emit typedefs
    for decl in &defn.decls {
        match decl.as_ref() {
            Decl::Fn(_) => (), // ignore
            Decl::Struct(decl) => emit_typedef(out, &decl.name)?,
            Decl::Opaque(decl) => emit_typedef(out, &decl.name)?,
        }
    }
    write!(out, "\n")?;

    // emit decls
    for decl in &defn.decls {
        match decl.as_ref() {
            Decl::Fn(decl) => emit_fn(out, decl)?,
            Decl::Struct(decl) => emit_struct(out, decl)?,
            Decl::Opaque(_) => (), // ignore
        }
    }
    write!(out, "{}", POSTAMBLE)?;
    Ok(())
}
