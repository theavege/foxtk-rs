set -euo pipefail
source '/etc/os-release'
case ${ID:?} in
    debian | ubuntu) sudo bash -c '
        apt-get update
        apt-get install -y libfox-1.6-dev
    ';;
    fedora | alma) sudo apt-get install -y fox-devel ;;
esac 1> /dev/null
clang-tidy 'foxtk-sys/src/foxtk.cpp' -- "$(pkg-config --cflags fox)"
cargo clippy --quiet --features="all" --examples
cargo build --release --features="all" --examples
cargo fmt --check --all
