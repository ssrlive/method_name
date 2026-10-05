#!/usr/bin/env bash

set -euxo pipefail

if [[ $# -ne 1 ]]; then
    printf 'Usage: %s <version>\n' "$0" >&2
    exit 2
fi

version=$1
if [[ ! $version =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z.-]+)?(\+[0-9A-Za-z.-]+)?$ ]]; then
    printf 'Invalid version: %s\n' "$version" >&2
    exit 2
fi

find . \
    -type d \( -name .git -o -name target \) -prune -o \
    -type f -name 'Cargo.toml' \
    -exec sed -i -E \
        "s/(version = \")[^\"]+(\"[[:space:]]+# Keep in sync)/\\1${version}\\2/" \
        {} +

cargo update -v -p method_name
