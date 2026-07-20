#!/usr/bin/env bash

set -euo pipefail

shellcheck --external-sources "${0}"
shfmt -ci -fn -i 4 -d "${0}"

if ! (pkg-config --cflags 'fox'); then
    source '/etc/os-release'
    case ${ID:?} in
        debian | ubuntu) sudo bash -c '
            apt-get update
            apt-get install -y shfmt libfox-1.6-dev
        ' ;;
        fedora | alma) sudo dnf install -y shfmt fox-devel ;;
    esac 1>/dev/null
fi

declare -r CSRC="foxtk-sys/src"
cppcheck "$(pkg-config --cflags fox)" "${CSRC:?}"/*.{cpp,h}
clang-tidy "${CSRC:?}"/*.{cpp,h} -- "$(pkg-config --cflags fox)"
clang-format --dry-run --Werror -style=Mozilla "${CSRC:?}"/*.{cpp,h}

cargo clippy --quiet --features="all" --examples
cargo build --release --features="all" --examples
cargo fmt --check --all
