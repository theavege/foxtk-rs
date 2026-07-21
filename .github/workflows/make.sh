#!/usr/bin/env bash

set -euo pipefail

if ! (command -v 'fox-config'); then
    source '/etc/os-release'
    case ${ID:?} in
        debian | ubuntu) sudo bash -c '
            apt-get update
            apt-get install -y shfmt cppcheck libfox-1.6-dev
        ' ;;
        fedora | alma) sudo dnf install -y shfmt cppcheck fox-devel ;;
    esac 1>/dev/null
fi

shellcheck --external-sources "${0}"
shfmt -ci -fn -i 4 -d "${0}"

declare -r CSRC="foxtk-sys/src"
cppcheck "$(fox-config --cflags)" "${CSRC:?}"/*.{cpp,h}
clang-tidy "${CSRC:?}"/*.{cpp,h} -- "$(fox-config --cflags)"
clang-format --dry-run --Werror -style=Mozilla "${CSRC:?}"/*.{cpp,h}

cargo clippy --quiet --features="all" --examples
cargo build --release --features="all" --examples
cargo fmt --check --all
