#!/bin/bash
set -e
THISDIR=$(dirname $(realpath $0))
cd $THISDIR

GEN=../../target/release/dandiya
OUT_DIR=out
GEN_DIR=$OUT_DIR/gen

# setup
rm -fr $OUT_DIR
mkdir -p $OUT_DIR
mkdir -p $GEN_DIR

# generate bindings for both C and Rust
$GEN example.dy -e c-header > $GEN_DIR/example.h
$GEN example.dy -e rust > $GEN_DIR/example.rs

# build libexample.a
gcc -o $OUT_DIR/impl_lib.o -c -fPIC -Wall -Werror -I$GEN_DIR impl_lib.c
ar rc $OUT_DIR/libexample.a $OUT_DIR/impl_lib.o

# build bin example
rustc --edition=2021 --crate-name example --crate-type lib \
      --emit=dep-info,metadata,link \
      --out-dir out \
      $GEN_DIR/example.rs

rustc --edition=2021 --crate-name example --crate-type bin \
      --extern example=out/libexample.rlib \
      -l example -L out \
      --out-dir out \
      impl_bin.rs
