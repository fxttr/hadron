#!/bin/sh

# var
OS="FreeBSD"

if [ ! $(command -v zypper) ]; then
  OS="OpenSUSE"
elif [ ! $(command -v apt) ]; then
  OS="Debian"
fi

# fn
function build_kernel() {
    cargo build
}

function build_image() {
    echo "IMG"
}

# main
build_kernel

if [ $1 == "IMAGE" ]; then
  build_image
fi