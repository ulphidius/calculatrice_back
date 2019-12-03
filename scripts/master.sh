#!/bin/bash

git tag ${VERSION}
git push origin ${VERSION}
sed -i "s|version = \"[[:digit:]][[:digit:]]*.[[:digit:]][[:digit:]]*.[[:digit:]][[:digit:]]*\"|version = \"${VERSION}\"|" ${CARGO_FILE}
