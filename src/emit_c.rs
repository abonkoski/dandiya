use crate::ast::*;

fn type_str(t: &Type) -> String {
    let (base, is_ptr) = match t {
        Type::None => return "void".to_string(),
        Type::Pointer(base) => (base, true),
        Type::Value(base) => (base, false),
    };

    let base_str = match base {
        BaseType::Struct(s) => format!("struct {}", s),
        BaseType::U8 => "uint8_t".to_string(),
        BaseType::I8 => "int8_t".to_string(),
        BaseType::U16 => "uint16_t".to_string(),
        BaseType::I16 => "int16_t".to_string(),
        BaseType::U32 => "uint32_t".to_string(),
        BaseType::I32 => "int32_t".to_string(),
        BaseType::U64 => "uint64_t".to_string(),
        BaseType::I64 => "int64_t".to_string(),
    };

    let mut s = String::new();
    s += &base_str;
    if is_ptr {
        s += " *";
    }

    s
}

fn args_str(args: &[Field]) -> String {
    let mut s = String::new();
    for f in args {
        if !s.is_empty() {
            s += ", ";
        }
        s += &format!("{} {}", type_str(&f.typ), f.name);
    }
    s
}

fn emit_fn(decl: &FuncDecl) {
    println!(
        "{} {}_v{}({});",
        type_str(&decl.ret),
        decl.name,
        decl.version,
        args_str(&decl.args)
    );
}

fn emit_struct(decl: &StructDecl) {
    println!("struct {} {{", decl.name);
    for f in &decl.fields {
        println!("    {:10} {};", type_str(&f.typ), f.name);
    }
    println!("}}");
}

pub fn emit(defn: &ApiDefn) {
    println!("#pragma once");
    println!("#include <stdint.h>");
    println!("");
    for decl in &defn.decls {
        match decl {
            Decl::Fn(decl) => emit_fn(decl),
            Decl::Struct(decl) => emit_struct(decl),
        }
    }
}
