# Example: C Binary calling Rust Library V2

An example showing version upgrades

## Files:

| Path | Description |
|------|-------------|
| `example.dy` | API Definition File |
|`impl_lib.rs` | Implementation of `out/libexample.a` in Rust |
| `impl_bin.c` | Implementation of `out/example` in C |
| `out/` | Directory for build artifacts |

## Comparing

Command: `diff . ../03_c_binary_calling_rust_library`

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
