.PHONY: build run

all: build run

build:
	wasm-pack build
	bash -c "cp -R ./pkg site-react/src/ && cd site-react/ && npm install"

run:
	bash -c "cd site-react/ && npm start"
