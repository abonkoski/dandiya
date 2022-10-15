use crate::ast::*;
use crate::{Error, Result};

#[derive(Debug, PartialEq)]
pub enum Token {
    Ident(String),
    Fn,
    Struct,
    Arrow,
    Punc(char),
    EndOfFile,
}

impl Token {
    fn take(&mut self) -> Token {
        std::mem::replace(self, Token::EndOfFile)
    }
}

pub struct Tokenizer {
    input: String,
    idx: usize,
    line_start_idx: usize,
    line_num: usize,
    srcname: Option<String>,
}

impl Tokenizer {
    pub fn new(inp: &str, srcname: Option<&str>) -> Self {
        Self {
            input: inp.to_string(),
            idx: 0,
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
        let line_off = self.idx - self.line_start_idx;
        let mut s = String::new();
        s += &format!(
            "{}:{}:{}: {}\n",
            self.srcname(),
            self.line_num,
            line_off + 1,
            msg
        );
        s += &format!("  {}\n", self.current_line());
        s += &format!("  {0:1$}^\n", "", self.idx - self.line_start_idx);
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
    c == '*' || c == ':' || c == ',' || c == ';'
}

fn tok_ident_or_keyword(s: String) -> Token {
    match &s as &str {
        "fn" => Token::Fn,
        "struct" => Token::Struct,
        _ => Token::Ident(s),
    }
}

pub struct Parser {
    tokenizer: Tokenizer,
    tok: Token,
}

macro_rules! expect {
    ($self: expr, $expr: pat) => {
        if !matches!($self.tok, $expr) {
            Err($self.tokenizer.error(&format!(
                "expected {}, found {:?}",
                stringify!($expr),
                $self.tok
            )))
        } else {
            $self.next_tok()?;
            Ok(())
        }
    };
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

    fn expect_ident(&mut self) -> Result<String> {
        if let Token::Ident(name) = self.tok.take() {
            self.next_tok()?;
            Ok(name)
        } else {
            Err(self.tokenizer.error(&format!(
                "expected {}, found {:?}",
                stringify!(Token::Ident(_)),
                self.tok,
            )))
        }
    }

    // type = "*"? base_type
    fn parse_type(&mut self) -> Result<Type> {
        let mut is_ptr = false;
        if matches!(self.tok, Token::Punc('*')) {
            is_ptr = true;
            self.next_tok()?;
        }
        let type_str = self.expect_ident()?;
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
        if is_ptr {
            Ok(Type::Pointer(base))
        } else {
            Ok(Type::Value(base))
        }
    }

    // field = ident ":" type
    fn maybe_parse_field(&mut self) -> Result<Option<Field>> {
        if !matches!(self.tok, Token::Ident(_)) {
            return Ok(None);
        }
        let name = self.expect_ident()?;
        expect!(self, Token::Punc(':'))?;
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
    fn parse_ret(&mut self) -> Result<Type> {
        if !matches!(self.tok, Token::Arrow) {
            return Ok(Type::None);
        }
        self.next_tok()?;
        self.parse_type()
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

    // func = "fn" "(" ident ")" ident "(" args ")" ret
    fn parse_fn(&mut self) -> Result<Decl> {
        expect!(self, Token::Fn)?;
        expect!(self, Token::Punc('('))?;
        let version = self.parse_version()?;
        expect!(self, Token::Punc(')'))?;
        let name = self.expect_ident()?;
        expect!(self, Token::Punc('('))?;
        let args = self.parse_fields()?;
        expect!(self, Token::Punc(')'))?;
        let ret = self.parse_ret()?;
        expect!(self, Token::Punc(';'))?;
        Ok(Decl::Fn(FuncDecl {
            name,
            args,
            ret,
            version,
        }))
    }

    // struct = "struct" ident "{" fields "}"
    fn parse_struct(&mut self) -> Result<Decl> {
        expect!(self, Token::Struct)?;
        let name = self.expect_ident()?;
        expect!(self, Token::Punc('{'))?;
        let fields = self.parse_fields()?;
        expect!(self, Token::Punc('}'))?;
        Ok(Decl::Struct(StructDecl { name, fields }))
    }

    // decl = func | struct
    fn maybe_parse_decl(&mut self) -> Result<Option<Decl>> {
        match self.tok {
            Token::Fn => Ok(Some(self.parse_fn()?)),
            Token::Struct => Ok(Some(self.parse_struct()?)),
            _ => Ok(None),
        }
    }

    pub fn parse(&mut self) -> Result<ApiDefn> {
        let mut api = ApiDefn { decls: vec![] };
        while let Some(decl) = self.maybe_parse_decl()? {
            api.decls.push(decl);
        }
        expect!(self, Token::EndOfFile)?;
        Ok(api)
    }
}

pub fn parse(inp: &str, srcname: Option<&str>) -> Result<ApiDefn> {
    Parser::new(inp, srcname)?.parse()
}
