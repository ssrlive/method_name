#!/bin/sh

set -euxo pipefail

find . \
    -type f \
    -name 'Cargo.toml' \
    -print \
    -a \
    -exec \
        sed -i -E "s/\".*?\"(  # Keep in sync)/\"$1\"\\1/g" '{}' \
    \;

cargo update -v -p method_name
