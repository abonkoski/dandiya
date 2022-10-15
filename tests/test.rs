use hydra::*;

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
    assert_eq!(tok.next_tok(), Token::EndOfFile);
    assert_eq!(tok.next_tok(), Token::EndOfFile);
    assert_eq!(tok.next_tok(), Token::EndOfFile);
}

#[test]
fn test_tok_ident() {
    let mut tok = Tokenizer::new("foobar3_ghs", None);
    assert_eq!(tok.next_tok(), Token::Ident("foobar3_ghs".to_string()));
    assert_eq!(tok.next_tok(), Token::EndOfFile);

    let mut tok = Tokenizer::new("_blah", None);
    assert_eq!(tok.next_tok(), Token::Ident("_blah".to_string()));
    assert_eq!(tok.next_tok(), Token::EndOfFile);

    let mut tok = Tokenizer::new("_blah _foo", None);
    assert_eq!(tok.next_tok(), Token::Ident("_blah".to_string()));
    assert_eq!(tok.next_tok(), Token::Ident("_foo".to_string()));
    assert_eq!(tok.next_tok(), Token::EndOfFile);
}

#[test]
fn test_tok_ident_and_punc() {
    let mut tok = Tokenizer::new("_blah,foo23", None);
    assert_eq!(tok.next_tok(), Token::Ident("_blah".to_string()));
    assert_eq!(tok.next_tok(), Token::Punc(','));
    assert_eq!(tok.next_tok(), Token::Ident("foo23".to_string()));
    assert_eq!(tok.next_tok(), Token::EndOfFile);
}

#[test]
fn test_tok_ident_and_kw() {
    let mut tok = Tokenizer::new("fn,fn7[struct", None);
    assert_eq!(tok.next_tok(), Token::Fn);
    assert_eq!(tok.next_tok(), Token::Punc(','));
    assert_eq!(tok.next_tok(), Token::Ident("fn7".to_string()));
    assert_eq!(tok.next_tok(), Token::Punc('['));
    assert_eq!(tok.next_tok(), Token::Struct);
    assert_eq!(tok.next_tok(), Token::EndOfFile);
}

#[test]
fn test_parse_one_fn() {
    let mut parser = Parser::new("fn (v1) _blah67 ( ) ; ", None);
    parser.parse();
}

#[test]
fn test_parse_two_fns() {
    let s = "\
      fn (v1) _blah67 ( ) ;
      fn (v2) _blah67 ( ) ;
      fn (v234) _foo23_gh ( ) ;
    ";
    let mut parser = Parser::new(s, None);
    parser.parse();
}

#[test]
fn test_parse_fn_with_args() {
    let s = "\
      fn (v1) _blah67 ( blah: u64 ) ;
      fn (v2) _blah67 ( gh: u32 ) ;
      fn (v234) _foo23_gh ( ab :  i8 , xy : i16) ;
    ";
    let mut parser = Parser::new(s, None);
    parser.parse();
}

#[test]
fn test_parse_fn_with_args_and_ret() {
    let s = "\
      fn (v1) _blah67 ( blah: u64 ) -> u8;
      fn (v2) _blah67 ( gh: u32 ) -> u32;
      fn (v234) _foo23_gh ( ab :  i8 , xy : i16)->i64;
    ";
    let mut parser = Parser::new(s, None);
    parser.parse();
}

#[test]
fn test_parse_fn_with_args_and_ret_and_ptrs() {
    let s = "\
      fn (v1) _blah67 ( blah: u64 ) -> *u8;
      fn (v2) _blah67 ( gh: u32 ) -> u32;
      fn (v234) _foo23_gh ( ab :  i8 , xy : * i16)->*i64;
    ";
    let mut parser = Parser::new(s, None);
    parser.parse();
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
    let mut parser = Parser::new(s, None);
    parser.parse();
}
