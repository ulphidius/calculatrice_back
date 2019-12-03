#!/bin/bash

VERSION="version = \"${MAJOR_VERSION}.${MINOR_VERSION}.${PATCH_VERSION}\""

echo ${VERSION}
echo ${CARGO_FILE}

git tag ${VERSION}
sed -i "s|version = \"[[:digit:]][[:digit:]]*.[[:digit:]][[:digit:]]*.[[:digit:]][[:digit:]]*\"|${VERSION}|" ${CARGO_FILE}
