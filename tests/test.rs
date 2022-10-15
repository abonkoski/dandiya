use hydra::parse::*;

#[test]
fn test_tok_char() {
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
fn test_tok_empty() {
    let mut tok = Tokenizer::new("", None);
    assert_eq!(tok.next_tok().unwrap(), Token::EndOfFile);
    assert_eq!(tok.next_tok().unwrap(), Token::EndOfFile);
    assert_eq!(tok.next_tok().unwrap(), Token::EndOfFile);
}

#[test]
fn test_tok_ident() {
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
fn test_tok_ident_and_punc() {
    let mut tok = Tokenizer::new("_blah,foo23", None);
    assert_eq!(tok.next_tok().unwrap(), Token::Ident("_blah".to_string()));
    assert_eq!(tok.next_tok().unwrap(), Token::Punc(','));
    assert_eq!(tok.next_tok().unwrap(), Token::Ident("foo23".to_string()));
    assert_eq!(tok.next_tok().unwrap(), Token::EndOfFile);
}

#[test]
fn test_tok_ident_and_kw() {
    let mut tok = Tokenizer::new("fn,fn7[struct", None);
    assert_eq!(tok.next_tok().unwrap(), Token::Fn);
    assert_eq!(tok.next_tok().unwrap(), Token::Punc(','));
    assert_eq!(tok.next_tok().unwrap(), Token::Ident("fn7".to_string()));
    assert_eq!(tok.next_tok().unwrap(), Token::Punc('['));
    assert_eq!(tok.next_tok().unwrap(), Token::Struct);
    assert_eq!(tok.next_tok().unwrap(), Token::EndOfFile);
}

#[test]
fn test_parse_one_fn() {
    parse("fn (v1) _blah67 ( ) ; ", None).unwrap();
}

#[test]
fn test_parse_two_fns() {
    let s = "\
      fn (v1) _blah67 ( ) ;
      fn (v2) _blah67 ( ) ;
      fn (v234) _foo23_gh ( ) ;
    ";
    parse(s, None).unwrap();
}

#[test]
fn test_parse_fn_with_args() {
    let s = "\
      fn (v1) _blah67 ( blah: u64 ) ;
      fn (v2) _blah67 ( gh: u32 ) ;
      fn (v234) _foo23_gh ( ab :  i8 , xy : i16) ;
    ";
    parse(s, None).unwrap();
}

#[test]
fn test_parse_fn_with_args_and_ret() {
    let s = "\
      fn (v1) _blah67 ( blah: u64 ) -> u8;
      fn (v2) _blah67 ( gh: u32 ) -> u32;
      fn (v234) _foo23_gh ( ab :  i8 , xy : i16)->i64;
    ";
    parse(s, None).unwrap();
}

#[test]
fn test_parse_fn_with_args_and_ret_and_ptrs() {
    let s = "\
      fn (v1) _blah67 ( blah: u64 ) -> *u8;
      fn (v2) _blah67 ( gh: u32 ) -> u32;
      fn (v234) _foo23_gh ( ab :  i8 , xy : * i16)->*i64;
    ";
    parse(s, None).unwrap();
}

#[test]
fn test_parse_struct() {
    let s = "\
      struct Foobar {
        foo: u8,
        bar: i32,
        baz: u16,
      }
     ";
    parse(s, None).unwrap();
}
