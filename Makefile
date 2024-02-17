.PHONY : dev

dev:
	cargo watch -x run

clippy:
	cargo clippy

build:
	cargo build

build-r:
	cargo build --release

.DEFAULT_GOAL := dev