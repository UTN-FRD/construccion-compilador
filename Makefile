.PHONY: build run

all: build run

build:
	wasm-pack build
	bash -c "cd site/ && npm install"

run:
	bash -c "cd site/ && npm run start"


clippy:
	cargo +nightly clippy
