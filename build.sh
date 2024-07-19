#!/bin/bash
set -ex

CLANG="${CLANG:-clang-18}"

cd deps/musl
CLANG=$CLANG ./ckb/build.sh
cd ../..

cd deps/builtins
make CC=$CLANG \
  LD=${CLANG/clang/ld.lld} \
  OBJCOPY=${CLANG/clang/llvm-objcopy} \
  AR=${CLANG/clang/llvm-ar} \
  RANLIB=${CLANG/clang/llvm-ranlib}
cd ../..
