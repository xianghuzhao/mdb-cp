#!/bin/sh

set -ex

if [ -z "$1" ]
then
    echo "Target must be provided."
    exit 1
fi

target="$1"

HOST=$(rustc -Vv | grep ^host: | sed -e "s/host: //g")
if [ "$HOST" != "$TARGET" ]
then
		rustup target install "$target"
fi

rustup -V
rustc -Vv
cargo -V
