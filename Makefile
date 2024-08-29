init:
	pip install pre-commit
	cargo clean && cargo verify-project && cargo fetch
build:
	cargo fmt
	cargo build --release
