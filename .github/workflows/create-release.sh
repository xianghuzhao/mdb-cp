#!/bin/sh

set -ex

if [ -z "$MDB_CP_REF_NAME" ]
then
    echo "Tag must be provided."
    exit 1
fi

tag="$MDB_CP_REF_NAME"
version=$(echo $tag | sed 's/^v*//')

awk "/^## ${version}$/{ f = 1; next } /^## /{ f = 0 } f" CHANGELOG.md | gh release create "${tag}" --title "Version ${version}" --notes-file -
