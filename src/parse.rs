use crate::ast::*;
use crate::{Error, Result};

use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum Token {
    EndOfFile,
    Ident(String),
    U64(u64),
    Fn,
    Struct,
    Opaque,
    Const,
    Arrow,
    Punc(char),
}

#[rustfmt::skip]
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::EndOfFile => write!(f, "<end-of-file>"),
            Token::Ident(_)  => write!(f, "<identifier>"),
            Token::U64(_)    => write!(f, "<u64>"),
            Token::Fn        => write!(f, "'fn'"),
            Token::Struct    => write!(f, "'struct'"),
            Token::Opaque    => write!(f, "'opaque'"),
            Token::Const     => write!(f, "'const'"),
            Token::Arrow     => write!(f, "'->'"),
            Token::Punc(c)   => write!(f, "'{}'", c),
        }
    }
}

pub struct Tokenizer {
    input: String,
    idx: usize,
    tok_idx: usize,
    line_start_idx: usize,
    line_num: usize,
    srcname: Option<String>,
}

impl Tokenizer {
    pub fn new(inp: &str, srcname: Option<&str>) -> Self {
        Self {
            input: inp.to_string(),
            idx: 0,
            tok_idx: 0,
            line_start_idx: 0,
            line_num: 1,
            srcname: srcname.map(str::to_string),
        }
    }

    pub fn peek_char(&mut self) -> Option<char> {
        self.input[self.idx..].chars().next()
    }

    pub fn advance_char(&mut self) {
        match self.peek_char() {
            Some(c) => self.idx += c.len_utf8(),
            None => (),
        }
    }

    pub fn next_tok(&mut self) -> Result<Token> {
        loop {
            self.tok_idx = self.idx;

            let c = match self.peek_char() {
                Some(c) => c,
                None => return Ok(Token::EndOfFile),
            };

            // newlines
            if c == '\n' {
                self.advance_char();
                self.line_start_idx = self.idx;
                self.line_num += 1;
                continue;
            }

            // skip whitespace
            if c.is_whitespace() {
                self.advance_char();
                continue;
            }

            // token '->'
            if c == '-' {
                self.advance_char();
                if self.peek_char() != Some('>') {
                    return Err(self.error("expected '->'"));
                }
                self.advance_char();
                return Ok(Token::Arrow);
            }

            // token single char punctuation
            if is_punc(c) {
                self.advance_char();
                return Ok(Token::Punc(c));
            }

            // parse u64 number
            if c.is_ascii_digit() {
                while let Some(c) = self.peek_char() {
                    if !c.is_ascii_digit() {
                        break;
                    }
                    self.advance_char();
                }
                let s = &self.input[self.tok_idx..self.idx];
                let n: u64 = match s.parse() {
                    Ok(n) => n,
                    Err(_) => {
                        return Err(self.error(&format!(
                            "tokenizer read a number that was too large for u64: '{}'",
                            s
                        )));
                    }
                };
                return Ok(Token::U64(n));
            }

            // parse identifier or keyword
            if is_ident_char_start(c) {
                let mut s = String::new();
                s.push(c);
                self.advance_char();
                while let Some(c) = self.peek_char() {
                    if !is_ident_char(c) {
                        break;
                    }
                    s.push(c);
                    self.advance_char();
                }
                return Ok(tok_ident_or_keyword(s));
            }

            return Err(self.error(&format!("tokenizer read an invalid character: '{}'", c)));
        }
    }

    fn current_line(&self) -> &str {
        let line_start = &self.input[self.line_start_idx..];
        match line_start.find('\n') {
            None => line_start,
            Some(idx) => &line_start[..idx],
        }
    }

    fn srcname(&self) -> &str {
        match &self.srcname {
            Some(name) => name,
            None => "(anonymous)",
        }
    }

    fn error(&self, msg: &str) -> Error {
        let tok_idx = self.tok_idx;
        let line_off = tok_idx - self.line_start_idx;
        let mut s = String::new();
        s += &format!(
            "{}:{}:{}: {}\n",
            self.srcname(),
            self.line_num,
            line_off + 1,
            msg
        );
        s += &format!("  {}\n", self.current_line());
        s += &format!("  {0:1$}^\n", "", tok_idx - self.line_start_idx);
        Error::ParseFailure(s)
    }
}

fn is_ident_char_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_ident_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

#[rustfmt::skip]
fn is_punc(c: char) -> bool {
    c == '[' || c == ']' || c == '(' || c == ')' || c == '{' || c == '}' ||
    c == '*' || c == ':' || c == ',' || c == ';' || c == '='
}

fn tok_ident_or_keyword(s: String) -> Token {
    match &s as &str {
        "fn" => Token::Fn,
        "struct" => Token::Struct,
        "opaque" => Token::Opaque,
        "const" => Token::Const,
        _ => Token::Ident(s),
    }
}

pub struct Parser {
    tokenizer: Tokenizer,
    tok: Token,
}

impl Parser {
    pub fn new(inp: &str, srcname: Option<&str>) -> Result<Self> {
        let mut tokenizer = Tokenizer::new(inp, srcname);
        let tok = tokenizer.next_tok()?;
        Ok(Self { tokenizer, tok })
    }

    fn next_tok(&mut self) -> Result<()> {
        self.tok = self.tokenizer.next_tok()?;
        Ok(())
    }

    fn expect(&mut self, expected: Token) -> Result<()> {
        if self.tok != expected {
            Err(self
                .tokenizer
                .error(&format!("expected {}, found {}", expected, self.tok)))
        } else {
            self.next_tok()?;
            Ok(())
        }
    }

    fn expect_ident(&mut self) -> Result<String> {
        if let Token::Ident(name) = &self.tok {
            let name = name.clone();
            self.next_tok()?;
            Ok(name)
        } else {
            Err(self.tokenizer.error(&format!(
                "expected {}, found {}",
                Token::Ident("".to_string()),
                self.tok,
            )))
        }
    }

    fn expect_u64(&mut self) -> Result<u64> {
        if let Token::U64(num) = &self.tok {
            let num = *num;
            self.next_tok()?;
            Ok(num)
        } else {
            Err(self
                .tokenizer
                .error(&format!("expected {}, found {}", Token::U64(0), self.tok,)))
        }
    }

    // basetype = ident | "u8" | "i8" | ... etc ...
    fn parse_basetype(&mut self) -> Result<BaseType> {
        if !matches!(self.tok, Token::Ident(_)) {
            return Err(self
                .tokenizer
                .error(&format!("expected <typename>, found {}", self.tok,)));
        }
        let type_str = self.expect_ident().unwrap(); // already checked
        let base = match &type_str as &str {
            "u8" => BaseType::U8,
            "i8" => BaseType::I8,
            "u16" => BaseType::U16,
            "i16" => BaseType::I16,
            "u32" => BaseType::U32,
            "i32" => BaseType::I32,
            "u64" => BaseType::U64,
            "i64" => BaseType::I64,
            _ => BaseType::Struct(type_str),
        };
        Ok(base)
    }

    // type = "*" type | "[" type ";" number "]" basetype
    fn parse_type(&mut self) -> Result<Type> {
        let typ = if matches!(self.tok, Token::Punc('*')) {
            self.next_tok()?;
            let typ = self.parse_type()?;
            Type::Pointer(Box::new(typ))
        } else if matches!(self.tok, Token::Punc('[')) {
            self.next_tok()?;
            let typ = self.parse_type()?;
            self.expect(Token::Punc(';'))?;
            let num = self.expect_u64()?;
            self.expect(Token::Punc(']'))?;
            Type::Array(Box::new(typ), num)
        } else {
            let typ = self.parse_basetype()?;
            Type::Base(typ)
        };

        if !type_is_sane_for_c(&typ) {
            return Err(self
                .tokenizer
                .error("type is too complex to express in C code"));
        }

        Ok(typ)
    }

    // field = ident ":" type
    fn maybe_parse_field(&mut self) -> Result<Option<Field>> {
        if !matches!(self.tok, Token::Ident(_)) {
            return Ok(None);
        }
        let name = self.expect_ident()?;
        self.expect(Token::Punc(':'))?;
        let typ = self.parse_type()?;
        Ok(Some(Field { name, typ }))
    }

    // fields = "" | field ("," field)* ","?
    fn parse_fields(&mut self) -> Result<Vec<Field>> {
        let mut args = Vec::new();

        match self.maybe_parse_field()? {
            Some(f) => args.push(f),
            None => return Ok(args),
        }

        loop {
            if !matches!(self.tok, Token::Punc(',')) {
                return Ok(args);
            }
            self.next_tok()?;
            match self.maybe_parse_field()? {
                Some(f) => args.push(f),
                None => return Ok(args),
            }
        }
    }

    // ret = ("->" type)?
    fn parse_ret(&mut self) -> Result<ReturnType> {
        if !matches!(self.tok, Token::Arrow) {
            return Ok(ReturnType::None);
        }
        self.next_tok()?;
        let typ = self.parse_type()?;
        if type_is_array_recursively(&typ) {
            return Err(self
                .tokenizer
                .error("return type is not allowed to be an array"));
        }
        Ok(ReturnType::Some(typ))
    }

    // version = "v" number
    fn parse_version(&mut self) -> Result<usize> {
        let v = self.expect_ident()?;
        if v.chars().next() != Some('v') {
            return Err(self.tokenizer.error("not a version identifier"));
        }
        let version: usize = match v[1..].parse() {
            Ok(v) => v,
            Err(_) => return Err(self.tokenizer.error("not a version number")),
        };
        return Ok(version);
    }

    // func = "fn" "(" ident ")" ident "(" args ")" ret ";"
    fn parse_fn(&mut self) -> Result<Decl> {
        self.expect(Token::Fn)?;
        self.expect(Token::Punc('('))?;
        let version = self.parse_version()?;
        self.expect(Token::Punc(')'))?;
        let name = self.expect_ident()?;
        self.expect(Token::Punc('('))?;
        let args = self.parse_fields()?;
        self.expect(Token::Punc(')'))?;
        let ret = self.parse_ret()?;
        self.expect(Token::Punc(';'))?;
        Ok(Decl::Fn(FuncDecl {
            name,
            args,
            ret,
            version,
        }))
    }

    // struct = "struct" ident "{" fields "}"
    fn parse_struct(&mut self) -> Result<Decl> {
        self.expect(Token::Struct)?;
        let name = self.expect_ident()?;
        self.expect(Token::Punc('{'))?;
        let fields = self.parse_fields()?;
        self.expect(Token::Punc('}'))?;
        Ok(Decl::Struct(StructDecl { name, fields }))
    }

    // opaque = "opaque" ident ";"
    fn parse_opaque(&mut self) -> Result<Decl> {
        self.expect(Token::Opaque)?;
        let name = self.expect_ident()?;
        self.expect(Token::Punc(';'))?;
        Ok(Decl::Opaque(OpaqueDecl { name }))
    }

    // const = "const" ident "=" u64 ";"
    fn parse_const(&mut self) -> Result<Decl> {
        self.expect(Token::Const)?;
        let name = self.expect_ident()?;
        self.expect(Token::Punc('='))?;
        let val = self.expect_u64()?;
        self.expect(Token::Punc(';'))?;
        Ok(Decl::Const(ConstDecl { name, val }))
    }

    // decl = func | struct | opaque | const
    fn maybe_parse_decl(&mut self) -> Result<Option<Decl>> {
        match self.tok {
            Token::Fn => Ok(Some(self.parse_fn()?)),
            Token::Struct => Ok(Some(self.parse_struct()?)),
            Token::Opaque => Ok(Some(self.parse_opaque()?)),
            Token::Const => Ok(Some(self.parse_const()?)),
            _ => Ok(None),
        }
    }

    pub fn parse(&mut self) -> Result<ApiDefn> {
        let mut api = ApiDefn {
            symbols: HashMap::new(),
            decls: vec![],
        };
        while let Some(decl) = self.maybe_parse_decl()? {
            let decl = Rc::new(decl);
            let name = decl.name();

            if api.symbols.contains_key(&name) {
                return Err(self
                    .tokenizer
                    .error(&format!("duplicate symbol '{}'", name)));
            }

            api.symbols.insert(name, decl.clone());
            api.decls.push(decl);
        }
        self.expect(Token::EndOfFile)?;
        Ok(api)
    }
}

// Retricts allowed types such that they can be sanely representable in C
fn type_is_sane_for_c(t: &Type) -> bool {
    match t {
        Type::Array(subtype, _) => {
            // If the type is an array, don't allow subtypes to be array
            // multi-dim array support in C is sad, so just don't do it
            !type_is_array_recursively(subtype)
        }
        Type::Pointer(subtype) => {
            // Pointers to arrays are wierd in C because of array to
            // pointer decay. Even in type signatures where the syntax
            // is allowed, it's actually just a pointer. C doesn't actually
            // have a semantic concept of pointers to arrays
            !type_is_array_recursively(subtype)
        }
        Type::Base(_) => true, // always sane
    }
}

fn type_is_array_recursively(t: &Type) -> bool {
    match t {
        Type::Array(_, _) => true,
        Type::Pointer(subtype) => type_is_array_recursively(subtype),
        Type::Base(_) => false,
    }
}

pub fn parse(inp: &str, srcname: Option<&str>) -> Result<ApiDefn> {
    Parser::new(inp, srcname)?.parse()
}
