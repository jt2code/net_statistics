.PHONY: build clean

default_target: build

build:
	RUSTFLAGS="-C target-feature=+crt-static" cargo build --release

clean:
	cargo clean