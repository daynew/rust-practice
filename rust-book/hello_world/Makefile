all: build run

build:
	cargo check

run:
	cargo run

watch:
	find . -name "*.rs" | entr -c cargo run

clean:
	cargo clean
