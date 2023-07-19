#!/bin/sh

# var
OS="FreeBSD"
BUILD_DIR=./build
PROFILE=debug
CLANGFLAGS="-Wall -O2 -ffreestanding -nostdinc -nostdlib -mcpu=cortex-a72+nosimd"

if [ ! $(command -v zypper) ]; then
  OS="OpenSUSE"
fi

# fn
function compile() {
    clang --target=aarch64-elf $CLANGFLAGS -c ./src/arch/aarch64/boot/boot.S -o $BUILD_DIR/boot.o
    cargo build
}

function link() {
    ld -m aarch64elf -nostdlib -T src/link.ld $BUILD_DIR/boot.o target/aarch64-unknown-lumos/$PROFILE/liblumos_rtl4.a -o $BUILD_DIR/lumos
}

function build_kernel() {
    compile
    link
}

function build_image() {
    echo "IMG"
}

if [ ! -d "$BUILD_DIR" ]; then
    mkdir $BUILD_DIR
fi

"$@"