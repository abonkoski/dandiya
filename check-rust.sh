#!/bin/bash
set -e

if [ "$#" != 1 ]; then
    echo "usage: $0 <input>"
    exit 1
fi
inp=$1

tmp_dir=/tmp/dandiya_test
tmp_src=$tmp_dir/test.rs
tmp_obj=$tmp_dir/test.o

# setup
rm -fr $tmp_dir
mkdir -p $tmp_dir

# gen
./target/release/dandiya $inp -e rust > $tmp_src

# compile
rustc --crate-type staticlib --emit=obj -o $tmp_obj -- $tmp_src

# cleanup
rm -fr $tmp_dir

