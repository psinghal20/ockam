#!/usr/bin/env bash

source "$(dirname "$0")"/common.sh

CRATE="$1"
VERSION=$(crate_version $CRATE)

TITLE="$CRATE v$VERSION (rust crate)"
TAG="${CRATE}_v$VERSION"
TMP=$(mktemp)
cat<<EOF>>$TMP
* [Crate](https://crates.io/crates/$CRATE/$VERSION)
* [Documentation](https://docs.rs/$CRATE/$VERSION/$CRATE/)
* [CHANGELOG](https://github.com/ockam-network/ockam/blob/$TAG/implementations/rust/ockam/$CRATE/CHANGELOG.md)
EOF
git tag -s -a $TAG -m "Release ${CRATE} v$VERSION"
git push --tags
gh release create -t "$TITLE" -F $TMP $TAG
