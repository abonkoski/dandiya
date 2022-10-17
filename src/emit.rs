use super::ast::ApiDefn;

pub mod c;
pub mod rust;

pub enum Language {
    C,
    Rust,
}

#[derive(Clone)]
pub struct Options {
    pub api_forward_to_latest: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            api_forward_to_latest: true,
        }
    }
}

pub fn emit(api: &ApiDefn, lang: Language, options: Options) -> String {
    let mut out = String::new();
    match lang {
        Language::C => c::emit(&mut out, api, options).unwrap(),
        Language::Rust => rust::emit(&mut out, api, options).unwrap(),
    }
    out
}
