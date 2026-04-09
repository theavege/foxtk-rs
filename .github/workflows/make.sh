set -euo pipefail
source '/etc/os-release'
case ${ID:?} in
    debian | ubuntu) sudo apt-get install -y libfox-1.6-dev ;;
    fedora | alma) sudo apt-get install -y fox-devel ;;
esac &> /dev/null
cargo clippy --quiet --example simple
cargo build --release --example simple
cargo fmt --check --all
