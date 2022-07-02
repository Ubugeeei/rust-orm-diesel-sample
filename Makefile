MIGRATE_ARG = create_todos

orm_setup:
	diesel setup

diesel_generate:
	diesel migration generate ${MIGRATE_ARG}

diesel_migrate:
	diesel migration run

build:
	cargo build

run:
	cargo run
