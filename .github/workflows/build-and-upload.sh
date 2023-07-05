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


tag="$MDB_CP_REF_NAME"
version=$(echo $tag | sed 's/^v*//')

bin_name=mdb-cp
asset_dir='asset-upload'
target_name="$target-$version"
target_dir="$asset_dir/$target_name"
archive_name="$target_name.tar.gz"
archive_path="$asset_dir/$archive_name"

mkdir -p "$target_dir"

mv "target/$target/release/$bin_name" "$target_dir/$bin_name"

tar -C "$asset_dir" -czf "$archive_path" "$target_name"

gh release upload "$tag" "$archive_path" --clobber
