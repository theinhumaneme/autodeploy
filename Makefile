init:
	cargo clean && cargo verify-project && cargo fetch
build:
	cargo build --release
