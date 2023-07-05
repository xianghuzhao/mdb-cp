#!/bin/sh

cd $(dirname "$0")

set -ex

if [ -z "$1" ]
then
    echo "Target must be provided."
    exit 1
fi

target="$1"

./install-rust.sh "$target"

./build.sh "$target"

gh release list
