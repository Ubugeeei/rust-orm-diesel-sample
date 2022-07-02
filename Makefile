orm_setup:
	diesel setup

migrate:
	diesel migration generate

build:
	cargo build

run:
	cargo run
