# Example: C Binary calling Rust Library

Simple example showing a C binary calling a Rust library through a Dandiya API definition

## Files:

| Path | Description |
|------|-------------|
| `example.dy` | API Definition File |
|`impl_lib.rs` | Implementation of `out/libexample.a` in Rust |
| `impl_bin.c` | Implementation of `out/example` in C |
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
