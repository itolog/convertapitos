.PHONY : dev

dev:
	cargo watch -x run

prod:
	docker compose up -d --build

prod-stop:
	docker compose stop

prod-down:
	docker compose down

clippy:
	cargo clippy

build:
	cargo build

build-r:
	cargo build --release

.DEFAULT_GOAL := dev