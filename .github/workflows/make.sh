#!/usr/bin/env bash

function _setup
(
    if [[ -f '/etc/os-release' ]]; then
        source '/etc/os-release'
        if ! command -v fox-config >/dev/null; then
            declare -ra DEPS=(sh{fmt,ellcheck})
            case ${ID:?} in
                debian | ubuntu)
                    sudo apt-get update
                    sudo apt-get install -y "${DEPS[@]}" libfox-1.6-dev
                    ;;
                fedora | alma) sudo dnf install -y "${DEPS[@]}" fox-devel ;;
            esac 1>/dev/null
        fi
        shellcheck --external-sources "${0}"
        shfmt -ci -fn -i 4 -d "${0}"
    fi
)

function _clang
(
    #~ declare -ra CSRC=('foxtk-sys/src'/*.{cpp,h})
    clang++ -std=c++17 -Wall -Wextra -Wpedantic -O2 \
        -fvisibility=hidden -fstack-protector-strong -fPIC \
        "$(fox-config --cflags)" \
        -c 'foxtk-sys/src/foxtk.cpp' -o foxtk.o
    read -ra args < <(fox-config --libs)
    clang -std=c17 -Wall -Wextra -Wpedantic -O2 -lstdc++ "${args[@]}" \
        "$(fox-config --cflags)" '-Ifoxtk-sys/src' foxtk.o \
        'foxtk-sys/examples/simple.c' -o simple
    #~ clang-tidy -checks='readability-*,bugprone-*,performance-*' \
    #~ --warnings-as-errors='*' "${CSRC[@]}" \
    #~ -- "$(fox-config --cflags)"
    #~ clang-format --dry-run --Werror -style=Microsoft "${CSRC[@]}"
)

function _rust
(
    cargo build --release --features='all' --examples
    cargo clippy --quiet --features='all' --examples
    cargo fmt --check --all
)

set -xeuo pipefail

if ((${#})); then
    case ${1} in
        setup) _setup ;;
        build) _clang && _rust ;;
    esac
fi
