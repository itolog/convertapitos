.PHONY : dev

dev:
	cargo watch -x run

build:
	cargo build

build-r:
	cargo build --release

.DEFAULT_GOAL := dev