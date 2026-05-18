set -euo pipefail
source '/etc/os-release'
case ${ID:?} in
    debian | ubuntu) sudo bash -c '
        apt-get update
        apt-get install -y libfox-1.6-dev
    ';;
    fedora | alma) sudo apt-get install -y fox-devel ;;
esac 1> /dev/null
cargo clippy --quiet --examples
cargo build --release --examples
cargo fmt --check --all
