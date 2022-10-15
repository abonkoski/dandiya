use dandiya::parse::*;

#[test]
fn tok_char() {
    let mut tok = Tokenizer::new("foz", None);
    assert_eq!(tok.peek_char(), Some('f'));
    assert_eq!(tok.peek_char(), Some('f'));
    tok.advance_char();
    assert_eq!(tok.peek_char(), Some('o'));
    assert_eq!(tok.peek_char(), Some('o'));
    tok.advance_char();
    assert_eq!(tok.peek_char(), Some('z'));
    assert_eq!(tok.peek_char(), Some('z'));
    tok.advance_char();
    assert_eq!(tok.peek_char(), None);
    assert_eq!(tok.peek_char(), None);
}

#[test]
fn tok_empty() {
    let mut tok = Tokenizer::new("", None);
    assert_eq!(tok.next_tok().unwrap(), Token::EndOfFile);
    assert_eq!(tok.next_tok().unwrap(), Token::EndOfFile);
    assert_eq!(tok.next_tok().unwrap(), Token::EndOfFile);
}

#[test]
fn tok_ident() {
    let mut tok = Tokenizer::new("foobar3_ghs", None);
    assert_eq!(
        tok.next_tok().unwrap(),
        Token::Ident("foobar3_ghs".to_string())
    );
    assert_eq!(tok.next_tok().unwrap(), Token::EndOfFile);

    let mut tok = Tokenizer::new("_blah", None);
    assert_eq!(tok.next_tok().unwrap(), Token::Ident("_blah".to_string()));
    assert_eq!(tok.next_tok().unwrap(), Token::EndOfFile);

    let mut tok = Tokenizer::new("_blah _foo", None);
    assert_eq!(tok.next_tok().unwrap(), Token::Ident("_blah".to_string()));
    assert_eq!(tok.next_tok().unwrap(), Token::Ident("_foo".to_string()));
    assert_eq!(tok.next_tok().unwrap(), Token::EndOfFile);
}

#[test]
fn tok_num() {
    let mut tok = Tokenizer::new("0", None);
    assert_eq!(tok.next_tok().unwrap(), Token::U64(0));
    assert_eq!(tok.next_tok().unwrap(), Token::EndOfFile);

    let mut tok = Tokenizer::new("123", None);
    assert_eq!(tok.next_tok().unwrap(), Token::U64(123));
    assert_eq!(tok.next_tok().unwrap(), Token::EndOfFile);

    let mut tok = Tokenizer::new("123456", None);
    assert_eq!(tok.next_tok().unwrap(), Token::U64(123456));
    assert_eq!(tok.next_tok().unwrap(), Token::EndOfFile);

    let mut tok = Tokenizer::new("123 456", None);
    assert_eq!(tok.next_tok().unwrap(), Token::U64(123));
    assert_eq!(tok.next_tok().unwrap(), Token::U64(456));
    assert_eq!(tok.next_tok().unwrap(), Token::EndOfFile);

    let mut tok = Tokenizer::new("18446744073709551615", None);
    assert_eq!(tok.next_tok().unwrap(), Token::U64(18446744073709551615));
    assert_eq!(tok.next_tok().unwrap(), Token::EndOfFile);

    // overflow u64
    let mut tok = Tokenizer::new("18446744073709551616", None);
    tok.next_tok().err().unwrap();

    // negative numbers not allowed currently
    let mut tok = Tokenizer::new("-1", None);
    tok.next_tok().err().unwrap();
}

#[test]
fn tok_ident_and_punc() {
    let mut tok = Tokenizer::new("_blah,foo23", None);
    assert_eq!(tok.next_tok().unwrap(), Token::Ident("_blah".to_string()));
    assert_eq!(tok.next_tok().unwrap(), Token::Punc(','));
    assert_eq!(tok.next_tok().unwrap(), Token::Ident("foo23".to_string()));
    assert_eq!(tok.next_tok().unwrap(), Token::EndOfFile);
}

#[test]
fn tok_ident_and_kw() {
    let mut tok = Tokenizer::new("fn,fn7[struct", None);
    assert_eq!(tok.next_tok().unwrap(), Token::Fn);
    assert_eq!(tok.next_tok().unwrap(), Token::Punc(','));
    assert_eq!(tok.next_tok().unwrap(), Token::Ident("fn7".to_string()));
    assert_eq!(tok.next_tok().unwrap(), Token::Punc('['));
    assert_eq!(tok.next_tok().unwrap(), Token::Struct);
    assert_eq!(tok.next_tok().unwrap(), Token::EndOfFile);
}

#[test]
fn parse_one_fn() {
    parse("fn (v1) _blah67 ( ) ; ", None).unwrap();
}

#[test]
fn parse_two_fns() {
    let s = "\
      fn (v1) _blah67 ( ) ;
      fn (v2) _blah67 ( ) ;
      fn (v234) _foo23_gh ( ) ;
    ";
    parse(s, None).unwrap();
}

#[test]
fn parse_fn_with_args() {
    let s = "\
      fn (v1) _blah67 ( blah: u64 ) ;
      fn (v2) _blah67 ( gh: u32 ) ;
      fn (v234) _foo23_gh ( ab :  i8 , xy : i16) ;
    ";
    parse(s, None).unwrap();
}

#[test]
fn parse_fn_with_args_and_ret() {
    let s = "\
      fn (v1) _blah67 ( blah: u64 ) -> u8;
      fn (v2) _blah67 ( gh: u32 ) -> u32;
      fn (v234) _foo23_gh ( ab :  i8 , xy : i16)->i64;
    ";
    parse(s, None).unwrap();
}

#[test]
fn parse_fn_with_args_and_ret_and_ptrs() {
    let s = "\
      fn (v1) _blah67 ( blah: u64 ) -> *u8;
      fn (v2) _blah67 ( gh: u32 ) -> u32;
      fn (v234) _foo23_gh ( ab :  i8 , xy : * i16)->*i64;
    ";
    parse(s, None).unwrap();
}

#[test]
fn parse_struct() {
    let s = "struct A {}";
    parse(s, None).unwrap();

    let s = "\
      struct Foobar {
        foo: u8,
        bar: i32,
        baz: u16,
      }
     ";
    parse(s, None).unwrap();
}

#[test]
fn parse_struct_with_ptr() {
    let s = "\
      struct Foobar {
        foo: u8,
        bar: *i32,
        baz: u16,
      }
     ";
    parse(s, None).unwrap();
}

#[test]
fn parse_struct_with_array() {
    let s = "\
      struct Foobar {
        foo: u8,
        bar: i32,
        baz: u16,
        arr: [u64; 42],
      }
     ";
    parse(s, None).unwrap();
}

#[test]
fn parse_struct_with_complex_types() {
    let s = "\
      struct Baz {
        ptr: *u8,
      }
      struct Foobar {
        foo: u8,
        bar: i32,
        baz: u16,
        arr: [u64; 42],
        complex: **[*[*Baz; 2]; 42],
      }
     ";
    // Parse rejects complex types that C can't handle
    parse(s, None).err().unwrap();
}

#[test]
fn parse_return_types() {
    let s = "\
       fn(v1) foo() -> *u64;
     ";
    parse(s, None).unwrap();

    let s = "\
       fn(v1) foo() -> [u64; 67];
     ";
    // Parse rejects array return types that C can't handle
    parse(s, None).err().unwrap();

    let s = "\
       fn(v1) foo() -> *[u64; 67];
     ";
    // Parse rejects array return types that C can't handle
    parse(s, None).err().unwrap();
}

#[test]
fn parse_duplicate_symbols() {
    let s = "struct A {} struct A {}";
    parse(s, None).err().unwrap();
}
