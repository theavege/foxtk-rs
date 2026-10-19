#!/usr/bin/env bash

function _setup
(
    if [[ -f '/etc/os-release' ]]; then
        source '/etc/os-release'
        if ! command -v fox-config || ! command -v ninja || ! command -v cmake || ! command -v pkg-config || ! command -v clang++ || ! command -v shellcheck || ! command -v shfmt; then
            case ${ID:?} in
                debian | ubuntu)
                    sudo apt-get update
                    sudo apt-get install -y shfmt shellcheck ninja-build cmake pkg-config clang libclang-dev build-essential libfox-1.6-dev xvfb
                    ;;
                fedora | alma)
                    sudo dnf install -y shfmt ShellCheck ninja-build cmake pkgconf-pkg-config clang llvm-devel gcc-c++ fox-devel xorg-x11-server-Xvfb
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
    if [[ "$(uname -s)" == Linux ]]; then
        "${cmd}" build --release --examples --features nobuild
    fi
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
