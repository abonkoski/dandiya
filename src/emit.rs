use super::ast::ApiDefn;

mod c;
mod rust;

pub enum Language {
    C,
    Rust,
}

pub fn emit(api: &ApiDefn, lang: Language) -> String {
    let mut out = String::new();
    match lang {
        Language::C => c::emit(&mut out, api).unwrap(),
        Language::Rust => rust::emit(&mut out, api).unwrap(),
    }
    out
}
