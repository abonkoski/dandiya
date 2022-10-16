#!/bin/bash
set -e

if [ "$#" != 1 ]; then
    echo "usage: $0 <input>"
    exit 1
fi
inp=$1

tmp_dir=/tmp/dandiya_test
tmp_hdr=$tmp_dir/test.h
tmp_src=$tmp_dir/test.c
tmp_obj=$tmp_dir/test.o

# setup
rm -fr $tmp_dir
mkdir -p $tmp_dir

# gen
./target/release/dandiya $inp -e c-header > $tmp_hdr

# gen driver
echo '#include "test.h"' > $tmp_src

# compile
gcc -c $tmp_src
clang -c $tmp_src

# cleanup
rm -fr $tmp_dir
