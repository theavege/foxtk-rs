#!/usr/bin/env bash

set -euo pipefail

workspace="$(pwd)"
temporary="$(mktemp -d)"
trap 'rm -rf "$temporary"' EXIT

mkdir -p "$temporary/fox/src" "$temporary/fox/include"
printf 'int fox_cmake_probe;\n' > "$temporary/fox/src/probe.cpp"

cmake -S "$workspace/foxtk-sys/src" \
    -B "$temporary/build" \
    -G Ninja \
    -DFOX_PATH="$temporary/fox"

test -f "$temporary/build/build.ninja"
grep -F "FOX_PATH:UNINITIALIZED=$temporary/fox" "$temporary/build/CMakeCache.txt"

printf 'FOX CMake configuration generated a Ninja build with FOX_PATH cached.\n'
