#!/bin/bash
set -e
THISDIR=$(dirname $(realpath $0))
cd $THISDIR

GEN=../target/release/dandiya
BUILD_DIR=build

# setup
rm -fr $BUILD_DIR
mkdir -p $BUILD_DIR

# generate bindings for both C and Rust
$GEN example.dy -e c-header > example.h
$GEN example.dy -e rust > example.rs

# build libexample.a
gcc -o $BUILD_DIR/lib_impl.o -c -fPIC -Wall -Werror lib_impl.c
ar rc $BUILD_DIR/libexample.a $BUILD_DIR/lib_impl.o

# build libexample.rlib
rustc --edition=2021 --crate-name example --crate-type lib \
      --emit=dep-info,metadata,link \
      --out-dir build \
      example.rs
rustc --edition=2021 --crate-name example --crate-type bin \
      --extern example=build/libexample.rlib \
      -l example -L build \
      --out-dir build \
      user_impl.rs
