#!/usr/bin/env bash

set -euo pipefail

if ! command -v fox-config >/dev/null; then
    source '/etc/os-release'
    case ${ID:?} in
        debian | ubuntu) sudo bash -c '
            apt-get update
            apt-get install -y shfmt cppcheck shellcheck libfox-1.6-dev
        ' ;;
        fedora | alma) sudo dnf install -y shfmt cppcheck shellcheck fox-devel ;;
    esac 1>/dev/null
fi

if command -v shellcheck >/dev/null; then
    shellcheck --external-sources "${0}"
else
    printf 'warning: shellcheck not installed; skipping shellcheck/n' >&2
fi

if command -v shfmt >/dev/null; then
    shfmt -ci -fn -i 4 -d "${0}"
else
    printf 'warning: shfmt not installed; skipping shfmt check\n' >&2
fi

declare -r CSRC="foxtk-sys/src"

#~ cppcheck "$(fox-config --cflags)" "${CSRC:?}"/*.{cpp,h}
clang-tidy "${CSRC:?}"/*.{cpp,h} -- "$(fox-config --cflags)"
if command -v clang-format >/dev/null; then
    clang-format --dry-run --Werror -style=Mozilla "${CSRC:?}"/*.{cpp,h}
else
    printf 'warning: clang-format not installed; skipping formatting check\n' >&2
fi

#~ clang++ "$(fox-config --cflags)" "${CSRC:?}/foxtk.cpp"
#~ clang "-I${CSRC:?}" 'foxtk-sys/examples/simple.c' -o simple.exe

cargo clippy --quiet --features="all" --examples
cargo build --release --features="all" --examples
cargo fmt --check --all
