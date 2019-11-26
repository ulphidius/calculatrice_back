#!/bin/bash
source ./env

VERSION="version = \"${MAJOR_VERSION}.${MINOR_VERSION}.${PATCH_VERSION}\""

echo ${VERSION}
echo ${CARGO_FILE}

sed -i "s|version = \"[[:digit:]][[:digit:]]*.[[:digit:]][[:digit:]]*.[[:digit:]][[:digit:]]*\"|${VERSION}|" ${CARGO_FILE}
#cargo login ${TOKEN_CRATE} && cargo deploy
