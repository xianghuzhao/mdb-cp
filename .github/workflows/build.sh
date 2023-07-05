#!/bin/sh

set -ex

if [ -z "$1" ]
then
    echo "Target must be provided."
    exit 1
fi

target="$1"

cargo build --release --target="${target}"
