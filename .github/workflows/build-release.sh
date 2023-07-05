#!/bin/sh

set -ex

if [ -z "$1" ]
then
    echo "Target must be provided."
    exit 1
fi

target="$1"

rustup target install "$target"

rustup -V
rustc -Vv
cargo -V

cargo build --release --target="${target}"

gh release list
