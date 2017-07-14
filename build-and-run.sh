#!/bin/bash

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

. $DIR/util/utils.sh

if [ -z "$TOOLCHAIN" ]; then
  . $DIR/toolchain/set-env-vars.sh
  echo "Setting toolchain environment variables"
fi

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

#echo "$PATH"

function build_toolchain() {
  echo ""
}

function build_kernel() {
  if [ ! -d kernel/build ]; then
    mkdir kernel/build
  fi
  pushd kernel/build
  if [ -n "$FLAG_D" ]; then
    cmake -DCMAKE_VERBOSE_MAKEFILE:BOOL=ON .. || bail
  else
    cmake .. || bail
  fi
  make install || bail
  popd
}

while getopts 'hfrd' flag; do
  case "${flag}" in
    r) FLAG_R='true' ;;
    f) FLAG_F='true' ;;
    d) FLAG_D='true' ;;
    h|\?)
      print_build_help
      exit 0
    ;;
    *)
      echo "Unexpected option ${flag}"
      exit 0
    ;;
  esac
done


if [ -d "$DIR/toolchain/build" ]; then
  echo "Toolchain already built, skipping..."
fi



if [ -n "$FLAG_R" ]; then
  if [ -n "$FLAG_F" ]; then
    rm -r $DIR/toolchain/build
    mkdir $DIR/toolchain/build
    build_toolchain || bail
  fi
  echo "HERE"
  rm -r $DIR/kernel/build
  mkdir $DIR/kernel/build
fi

echo "$DIR"

build_kernel


grub-mkrescue -o boringos-disk.img util || bail

qemu-system-x86_64 -cdrom boringos-disk.img || bail
