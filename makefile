watch:
	cargo watch -x check -x test -x run

lint:
	cargo fmt -- --check
	cargo clippy -- -D warnings
	cargo audit

test:
	cargo fmt
	cargo test
	cargo tarpaulin --ignore-tests

install-dx:
	cargo install cargo-watch
	cargo install cargo-tarpaulin
	cargo install cargo-audit
	cargo install cargo-expand

expand:
	cargo expand > .raw.rs