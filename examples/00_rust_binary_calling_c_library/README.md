# Example: Rust Binary calling C Library

Simple example showing a Rust binary calling a C library through a Dandiya API definition

## Files:

| Path | Description |
|------|-------------|
| `example.dy` | API Definition File |
|`impl_lib.c` | Implementation of `out/libexample.a` in C |
| `impl_bin.rs` | Implementation of `out/example` in Rust |
| `out/` | Directory for build artifacts |

## Building
Command: `./build.sh`

## Running
Command `./out/example`

Output:
```
batch 0: 1
batch 1: 14
batch 2: 11
batch 3: 8
batch 4: 5
```
