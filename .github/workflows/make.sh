#!/usr/bin/env bash

function _setup
(
    if [[ -f '/etc/os-release' ]]; then
        source '/etc/os-release'
        if ! command -v fox-config >/dev/null; then
            case ${ID:?} in
                debian | ubuntu) sudo bash -c '
                    apt-get update
                    apt-get install -y shfmt shellcheck libfox-1.6-dev
                ' ;;
                fedora | alma) sudo dnf install -y shfmt shellcheck fox-devel ;;
            esac 1>/dev/null
        fi
    fi
)

set -euo pipefail

if ((${#})); then
    case ${1} in
        setup) _setup ;;
        build)
            shellcheck --external-sources "${0}"
            shfmt -ci -fn -i 4 -d "${0}"

            declare -r CSRC="foxtk-sys/src"
            clang-tidy "${CSRC:?}"/*.{cpp,h} -- "$(fox-config --cflags)"
            clang-format --dry-run --Werror -style=Mozilla "${CSRC:?}"/*.{cpp,h}

            cargo clippy --quiet --features="all" --examples
            cargo build --release --features="all" --examples
            cargo fmt --check --all
            ;;
    esac
fi
