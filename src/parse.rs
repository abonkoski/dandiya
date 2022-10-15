use crate::ast::*;

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
}

impl Tokenizer {
    pub fn new(inp: &str) -> Self {
        Self {
            input: inp.to_string(),
            idx: 0,
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

    pub fn next_tok(&mut self) -> Token {
        loop {
            let c = match self.peek_char() {
                Some(c) => c,
                None => return Token::EndOfFile,
            };

            // skip whitespace
            if c.is_whitespace() {
                self.advance_char();
                continue;
            }

            if c == '-' {
                self.advance_char();
                if self.peek_char() != Some('>') {
                    panic!("Expected '->'");
                }
                self.advance_char();
                return Token::Arrow;
            }

            if is_punc(c) {
                self.advance_char();
                return Token::Punc(c);
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
                return tok_ident_or_keyword(s);
            }

            panic!("Invalid char: '{}'", c);
        }
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
            panic!("Expected {}, found {:?}", stringify!($expr), $self.tok);
        }
        $self.next_tok();
    };
}

impl Parser {
    pub fn new(inp: &str) -> Self {
        let mut tokenizer = Tokenizer::new(inp);
        let tok = tokenizer.next_tok();
        Self { tokenizer, tok }
    }

    fn next_tok(&mut self) {
        self.tok = self.tokenizer.next_tok();
    }

    fn expect_ident(&mut self) -> String {
        if let Token::Ident(name) = self.tok.take() {
            self.next_tok();
            name
        } else {
            panic!(
                "Expected {}, found {:?}",
                stringify!(Token::Ident(_)),
                self.tok
            );
        }
    }

    // type = "*"? base_type
    fn parse_type(&mut self) -> Type {
        let mut is_ptr = false;
        if matches!(self.tok, Token::Punc('*')) {
            is_ptr = true;
            self.next_tok();
        }
        let type_str = self.expect_ident();
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
            Type::Pointer(base)
        } else {
            Type::Value(base)
        }
    }

    // field = ident ":" type
    fn maybe_parse_field(&mut self) -> Option<Field> {
        if !matches!(self.tok, Token::Ident(_)) {
            return None;
        }
        let name = self.expect_ident();
        expect!(self, Token::Punc(':'));
        let typ = self.parse_type();
        Some(Field { name, typ })
    }

    // fields = "" | field ("," field)* ","?
    fn parse_fields(&mut self) -> Vec<Field> {
        let mut args = Vec::new();

        match self.maybe_parse_field() {
            Some(f) => args.push(f),
            None => return args,
        }

        loop {
            if !matches!(self.tok, Token::Punc(',')) {
                return args;
            }
            self.next_tok();
            match self.maybe_parse_field() {
                Some(f) => args.push(f),
                None => return args,
            }
        }
    }

    // ret = ("->" type)?
    fn parse_ret(&mut self) -> Type {
        if !matches!(self.tok, Token::Arrow) {
            return Type::None;
        }
        self.next_tok();
        self.parse_type()
    }

    // func = "fn" "(" ident ")" ident "(" args ")" ret
    fn parse_fn(&mut self) -> Decl {
        expect!(self, Token::Fn);
        expect!(self, Token::Punc('('));
        let version = parse_version(&self.expect_ident()).unwrap();
        expect!(self, Token::Punc(')'));
        let name = self.expect_ident();
        expect!(self, Token::Punc('('));
        let args = self.parse_fields();
        expect!(self, Token::Punc(')'));
        let ret = self.parse_ret();
        expect!(self, Token::Punc(';'));
        Decl::Fn(FuncDecl {
            name,
            args,
            ret,
            version,
        })
    }

    // struct = "struct" ident "{" fields "}"
    fn parse_struct(&mut self) -> Decl {
        expect!(self, Token::Struct);
        let name = self.expect_ident();
        expect!(self, Token::Punc('{'));
        let fields = self.parse_fields();
        expect!(self, Token::Punc('}'));
        Decl::Struct(StructDecl { name, fields })
    }

    // decl = func | struct
    fn maybe_parse_decl(&mut self) -> Option<Decl> {
        match self.tok {
            Token::Fn => Some(self.parse_fn()),
            Token::Struct => Some(self.parse_struct()),
            _ => None,
        }
    }

    pub fn parse(&mut self) -> ApiDefn {
        let mut api = ApiDefn { decls: vec![] };
        while let Some(decl) = self.maybe_parse_decl() {
            api.decls.push(decl);
        }
        expect!(self, Token::EndOfFile);
        api
    }
}

fn parse_version(v: &str) -> Result<usize> {
    if v.chars().next() != Some('v') {
        panic!("Not version identifier");
    }
    let version: usize = match v[1..].parse() {
        Ok(v) => v,
        Err(_) => panic!("Failed to parse version number"),
    };
    return Ok(version);
}
