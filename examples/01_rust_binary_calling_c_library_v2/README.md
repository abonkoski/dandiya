# Example: Rust Binary calling C Library V2

An example showing version upgrades

## Files:

| Path | Description |
|------|-------------|
| `example.dy` | API Definition File |
|`impl_lib.c` | Implementation of `out/libexample.a` in C |
| `impl_bin.rs` | Implementation of `out/example` in Rust |
| `out/` | Directory for build artifacts |

## Comparing

Command: `diff . ../00_rust_binary_calling_c_library`

## Building
Command: `./build.sh`

## Running
Command `./out/example`

Output:
```
batch 0: 2
batch 1: 15
batch 2: 12
batch 3: 9
batch 4: 6
```
