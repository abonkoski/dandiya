use dandiya::emit::*;
use dandiya::parse::*;

fn c_preamble() -> String {
    "\
#pragma once
#include <stdint.h>

"
    .to_string()
}

fn check(src: &str, emit_c: &str, emit_rust: &str) {
    let api = parse(src, None).unwrap();
    let c = emit(&api, Language::C);
    assert_eq!(c, c_preamble() + emit_c + "\n");
    let rust = emit(&api, Language::Rust);
    assert_eq!(rust, emit_rust.to_string() + "\n");
}

#[test]
fn emit_simple_func() {
    let src = "fn(v1) my_func(a: u8, b: u16) -> u64;";
    let c = "uint64_t my_func_v1(uint8_t a, uint16_t b);";
    let rust = "fn my_func_v1(a: u8, b: u16) -> u64";
    check(src, &c, rust);
}
