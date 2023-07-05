#!/bin/sh

set -ex

if [ -z "$1" ]
then
    echo "Tag must be provided."
    exit 1
fi

tag="$1"
version=$(echo $tag | sed 's/^v*//')

awk "/^## ${version}$/{ f = 1; next } /^## /{ f = 0 } f" CHANGELOG.md | gh release create "${tag}" --notes-file -
