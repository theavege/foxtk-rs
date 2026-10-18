#!/usr/bin/env bash

function _setup
(
    if [[ -f '/etc/os-release' ]]; then
        source '/etc/os-release'
        if ! command -v fox-config; then
            declare -ra DEPS=(sh{fmt,ellcheck})
            case ${ID:?} in
                debian | ubuntu)
                    sudo apt-get update
                    sudo apt-get install -y "${DEPS[@]}" libfox-1.6-dev xvfb
                    ;;
                fedora | alma)
                    sudo dnf install -y "${DEPS[@]}" fox-devel xorg-x11-server-Xvfb
                    ;;
            esac 1>/dev/null
        fi
        shellcheck --external-sources "${0}"
        shfmt -ci -fn -i 4 -d "${0}"
    fi
)

function _clang
(
    clang++ -std=c++17 -Wall -Wextra -Wpedantic -O2 \
        -fvisibility=hidden -fstack-protector-strong -fPIC \
        "$(fox-config --cflags)" \
        -c 'foxtk-sys/src/foxtk.cpp' -o foxtk.o
    read -ra args < <(fox-config --libs)
    clang -std=c17 -Wall -Wextra -Wpedantic -O2 -lstdc++ "${args[@]}" \
        "$(fox-config --cflags)" '-Ifoxtk-sys/src' foxtk.o \
        'foxtk-sys/examples/simple.c' -o simple
)

function _rust
(
    declare cmd=cargo
    if ! command -v cargo; then
        sudo apt install -y rustfmt-1.91 rust-1.91-clippy cargo-1.91
        cmd="$(command -v cargo-1.91)"
    fi
    "${cmd}" build --release --examples
    "${cmd}" clippy --quiet --examples
    "${cmd}" doc --no-deps
    "${cmd}" fmt --check --all
)

set -xeuo pipefail

if ((${#})); then
    case ${1} in
        setup) _setup ;;
        build) _clang && _rust ;;
    esac
fi
