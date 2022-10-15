use dandiya::emit::*;
use dandiya::parse::*;

fn check(src: &str, emit_c: &str, emit_rust: &str) {
    let api = parse(src, None).unwrap();
    let c = emit(&api, Language::C);
    assert_eq!(c, c::PREAMBLE.to_string() + "\n" + emit_c + "\n");
    let rust = emit(&api, Language::Rust);
    assert_eq!(rust, emit_rust.to_string() + "\n");
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
fn (v2) do_thing(dat: *data, p: u16) -> *u8;
";

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
