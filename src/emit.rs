use super::ast::ApiDefn;

mod c;
mod rust;

pub enum Language {
    C,
    Rust,
}

pub fn emit(api: &ApiDefn, lang: Language) {
    match lang {
        Language::C => c::emit(api),
        Language::Rust => rust::emit(api),
    }
}
