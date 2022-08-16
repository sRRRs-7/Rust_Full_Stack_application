DB_URL="postgresql://root:secret@localhost:5432/fullstack_app?sslmode=disable"

run:
	cargo run

check:
	cargo check

postgres:
	docker run --name rust_server -p 5432:5432 -e POSTGRES_USER=root -e POSTGRES_PASSWORD=secret -d postgres:14-alpine

createdb:
	docker exec -it rust_server createdb --username=root --owner=root fullstack_app

dropdb:
	docker exec -it rust_server dropdb fullstack_app


.PHONY: run, check, postgres, createdb, dropdb