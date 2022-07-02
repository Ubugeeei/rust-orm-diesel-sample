MIGRATE_ARG = create_todos

orm_setup:
	diesel setup --database-url ./db/sample.db

orm_generate:
	mkdir -p db && diesel migration generate ${MIGRATE_ARG} --database-url ./db/sample.db

orm_migrate:
	mkdir -p db && diesel migration run --database-url ./db/sample.db

orm_revert:
	diesel migration revert --database-url ./db/sample.db

orm_remigrate:
	make orm_revert
	make orm_migrate

build:
	cargo build

run:
	cargo run
