cargo test -p minifix
cargo clippy -p minifix
cargo fmt -- --check
cargo package -p minifix --list

