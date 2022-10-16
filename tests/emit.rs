use dandiya::emit::*;
use dandiya::parse::*;

fn check(src: &str, emit_c: &str, emit_rust: &str) {
    let api = parse(src, None).unwrap();

    let c = emit(&api, Language::C);
    let expected_c = format!("{}{}{}", c::PREAMBLE, emit_c, c::POSTAMBLE);
    assert_eq!(c, expected_c);

    let rust = emit(&api, Language::Rust);
    let expected_rust = format!("{}{}{}", rust::PREAMBLE, emit_rust, rust::POSTAMBLE);
    assert_eq!(rust, expected_rust);
}

#[test]
fn emit_simple_func() {
    let src = "fn(v1) my_func(a: u8, b: u16) -> u64;";
    let c = "uint64_t my_func_v1(uint8_t a, uint16_t b);";
    let rust = "extern \"C\" fn my_func_v1(a: u8, b: u16) -> u64;";
    check(src, &c, rust);
}

#[test]
fn emit_struct() {
    let src = "\
struct name {
  foo: *u64,
  bar: [u16; 4],
  baz: [**u8; 8],
}";

    let c = "\
typedef struct name name_t;
struct name {
  uint64_t* foo;
  uint16_t bar[4];
  uint8_t** baz[8];
};";

    let rust = "\
#[repr(C)]
struct name {
  foo: *mut u64,
  bar: [u16; 4],
  baz: [*mut *mut u8; 8],
}";

    check(src, &c, rust);
}

#[test]
fn emit_struct_and_func() {
    let src = "\
struct data {
  foo: i32,
  bar: i8,
}

fn (v1) do_thing(dat: *data) -> u8;
fn (v2) do_thing(dat: *data, p: u16) -> *u8;";

    let c = "\
typedef struct data data_t;
struct data {
  int32_t foo;
  int8_t bar;
};

uint8_t do_thing_v1(data_t* dat);
uint8_t* do_thing_v2(data_t* dat, uint16_t p);";

    let rust = "\
#[repr(C)]
struct data {
  foo: i32,
  bar: i8,
}

extern \"C\" fn do_thing_v1(dat: *mut data) -> u8;
extern \"C\" fn do_thing_v2(dat: *mut data, p: u16) -> *mut u8;";

    check(src, &c, rust);
}

#[test]
fn emit_opaque() {
    let src = "opaque mytype;";
    let c = "typedef struct mytype mytype_t;";
    let rust = "#[repr(C)]\nstruct mytype {_opaque_data: [u8; 0]}";

    check(src, &c, rust);
}

#[test]
fn emit_no_args() {
    let src = "fn(v1) func() -> u32;";
    let c = "uint32_t func_v1(void);";
    let rust = "extern \"C\" fn func_v1() -> u32;";

    check(src, &c, rust);
}

#[test]
fn emit_const() {
    let src = "const MYCONST = 4235;";
    let c = "#define MYCONST ((uint64_t)(4235))";
    let rust = "const MYCONST: u64 = 4235;";

    check(src, &c, rust);
}

#[test]
fn emit_skip() {
    // Test that skip text is respected
    let src = "
// Some comment here
/* multi
    line comment
*/ struct Foo {
}

/* comment */ // followed by another

fn(v1) foo();

  //Trailingcomment";

    let c = "
// Some comment here
/* multi
    line comment
*/ typedef struct Foo Foo_t;
struct Foo {
};

/* comment */ // followed by another

void foo_v1(void);

  //Trailingcomment";

    let rust = "
// Some comment here
/* multi
    line comment
*/ #[repr(C)]
struct Foo {
}

/* comment */ // followed by another

extern \"C\" fn foo_v1();

  //Trailingcomment";

    check(src, &c, rust);
}
