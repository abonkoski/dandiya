# Example: Rust Binary calling C Library V1 Stale

An example showing version upgrades where the binary is using a stale defn

## Files:

| Path | Description |
|------|-------------|
| `example.dy` | API Definition File |
| `example_stale.dy` | Stale API Definition File used by binary |
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
batch 0: 1
batch 1: 14
batch 2: 11
batch 3: 8
batch 4: 5
```
